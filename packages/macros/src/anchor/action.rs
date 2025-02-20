use anchor_lang_idl::types::{Idl, IdlInstruction};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Ident, Token,
};

use crate::{
    action::attribute::get_action_derivations,
    anchor::util::{
        convert_idl_type_def_to_ts, convert_idl_type_to_syn_type, find_account_index,
        gen_discriminator,
    },
};

use super::util::get_idl;

#[derive(Debug)]
pub struct DeclareAnchorActions {
    name: Ident,
    idl: Idl,
    instructions: Punctuated<Instruction, Comma>,
}

#[derive(Debug)]
pub struct Instruction {
    name: Ident,
    args: Option<Args>,
    accounts: Option<Accounts>,
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);

        let mut args = None;
        let mut accounts = None;

        while !content.is_empty() {
            let section_ident: Ident = content.parse()?;

            // Check for Args
            if section_ident == "Args" {
                content.parse::<Token![:]>()?;
                let args_content;
                braced!(args_content in content);
                args = Some(Args::parse(&args_content)?);
            }
            // Check for Accounts
            else if section_ident == "Accounts" {
                content.parse::<Token![:]>()?;
                let accounts_content;
                braced!(accounts_content in content);
                accounts = Some(Accounts::parse(&accounts_content)?);
            } else {
                return Err(syn::Error::new(
                    section_ident.span(),
                    "Expected 'Args' or 'Accounts'",
                ));
            }

            // Optionally, you can handle commas or other separators here
            if !content.is_empty() {
                content.parse::<Token![,]>().ok(); // Skip optional commas
            }
        }

        Ok(Instruction {
            name,
            args,
            accounts,
        })
    }
}

#[derive(Debug)]
struct Args {
    fields: Punctuated<Ident, Comma>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields = input.parse_terminated(Ident::parse)?;
        Ok(Args { fields })
    }
}

#[derive(Debug)]
struct Accounts {
    fields: Punctuated<Ident, Comma>,
}

impl Parse for Accounts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields = input.parse_terminated(Ident::parse)?;
        Ok(Accounts { fields })
    }
}

impl Parse for DeclareAnchorActions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let idl = get_idl(&name).map_err(|e| syn::Error::new(name.span(), e))?;

        input.parse::<Token![,]>()?;

        let instructions = input.parse_terminated(Instruction::parse)?;

        Ok(Self {
            name,
            idl,
            instructions,
        })
    }
}

pub fn gen_anchor_action_declarations(input: DeclareAnchorActions) -> TokenStream {
    let action_mod_name = format_ident!("{}_actions", input.name.to_string().to_snake_case());
    let anchor_mod_name = format_ident!("{}_anchor", input.name.to_string().to_snake_case());

    let idl = &input.idl;
    let mut action_structs = Vec::with_capacity(input.instructions.len());
    let mut deserialize_ixs = Vec::with_capacity(input.instructions.len());

    for ix in &input.instructions {
        // Find desired arguments and type from IDL
        let idl_ix = idl
            .instructions
            .iter()
            .find(|i| i.name == ix.name.to_string().to_snake_case())
            .expect(format!("Instruction {:?} not found in IDL", ix.name.to_string()).as_str());

        let deserialize_ix = gen_deserialize_ix_struct(&idl_ix);
        deserialize_ixs.push(deserialize_ix);

        let action_struct = gen_action_struct(input.name.to_string(), &idl_ix, &ix);
        action_structs.push(action_struct);
    }

    let types_mod = gen_types_mod(&idl);

    let actions_enum = gen_actions_enum(&input);

    let derivations = get_action_derivations();

    let expanded = quote! {
        pub mod #anchor_mod_name {
            use anchor_lang::prelude::*;

            pub mod internal {
                use super::types::*;
                use anchor_lang::prelude::*;

                #(#deserialize_ixs)*
            }

            #types_mod
        }

        pub mod #action_mod_name {
            use anchor_lang::{Discriminator, AnchorDeserialize};

            #(#action_structs)*
        }

        #derivations
        #actions_enum
    };

    expanded
}

/// Generate outward-facing action struct based on specified data to
/// deserialize from instruction data.
fn gen_action_struct(name: String, idl_ix: &IdlInstruction, ix: &Instruction) -> TokenStream {
    let ix_struct_name = format_ident!("{}", ix.name.to_string().to_upper_camel_case());

    let mut arg_fields = Vec::new();
    let mut arg_conversion_fields = Vec::new();

    // Add argument fields to struct definition
    if let Some(args) = &ix.args {
        for arg in &args.fields {
            let idl_arg = idl_ix
                .args
                .iter()
                .find(|a| a.name == arg.to_string())
                .expect(
                    format!(
                        "Argument {:?} not found in Instruction {:?}",
                        arg.to_string(),
                        ix.name,
                    )
                    .as_str(),
                );

            let arg_ty = convert_idl_type_to_syn_type(&idl_arg.ty);

            let arg_ident = Ident::new(&arg.to_string(), arg.span());
            arg_fields.push(quote! {
                pub #arg_ident: #arg_ty,
            });

            // Used for converting from internal anchor args to action struct
            arg_conversion_fields.push(quote! {
                #arg_ident: anchor_args.#arg_ident,
            });
        }
    }

    // Add account fields to struct definition
    let mut account_fields = Vec::new();
    let mut account_conversion_fields = Vec::new();
    if let Some(accts) = &ix.accounts {
        for field in &accts.fields {
            let account_name = field.to_string().to_snake_case();

            // Find position of field in anchor IDL
            let idl_acct_idx = find_account_index(&idl_ix, &account_name).expect(
                format!(
                    "Account {:?} not found in Instruction {:?}",
                    field.to_string(),
                    ix.name,
                )
                .as_str(),
            );

            let error_msg = format!("MissingAccount: {}", account_name);
            let error_msg_lit = syn::LitStr::new(&error_msg, proc_macro2::Span::call_site());

            let account_field = quote! {
                #field: txn.get_pubkey(ix.accounts[#idl_acct_idx])
                    .ok_or_else(|| anyhow::anyhow!(#error_msg_lit))?,
            };

            account_conversion_fields.push(account_field);
            account_fields.push(quote! {
                pub #field: solana_sdk::pubkey::Pubkey,
            });
        }
    }

    // From function to convert internally deserialized anchor account to
    // action struct
    let protocol_name_snake = format_ident!("{}_anchor", name.to_snake_case());

    let from_impl = quote! {
        impl #ix_struct_name {
            pub fn from_instruction(txn: &classifier_core::ClassifiableTransaction, ix: &classifier_core::ClassifiableInstruction) -> Result<Self, anyhow::Error> {
                let discriminator_len = super::#protocol_name_snake::internal::#ix_struct_name::DISCRIMINATOR.len();

                let anchor_args = super::#protocol_name_snake::internal::#ix_struct_name::deserialize(&mut &ix.data[discriminator_len..])
                    .map_err(|e| anyhow::anyhow!("Anchor deserialization error: {:?}", e))?;

                Ok(Self {
                    #(#arg_conversion_fields)*
                    #(#account_conversion_fields)*
                })
            }
        }
    };

    let ix_struct = if arg_fields.is_empty() {
        quote! {
            pub struct #ix_struct_name;
        }
    } else {
        quote! {
            pub struct #ix_struct_name {
                #(#arg_fields)*
                #(#account_fields)*
            }
        }
    };

    let derivations = get_action_derivations();

    quote! {
        #derivations
        #ix_struct

        #from_impl
    }
}

/// Mostly taken from anchor-lang's declare_program proc-macro,
/// Includes the necessary code to deserialize instruction data based on the
/// IDL spec.
fn gen_deserialize_ix_struct(ix: &IdlInstruction) -> TokenStream {
    let ix_struct_name = format_ident!("{}", ix.name.to_upper_camel_case());

    let fields = ix.args.iter().map(|arg| {
        let name = format_ident!("{}", arg.name);
        let ty = convert_idl_type_to_syn_type(&arg.ty);
        quote! { pub #name: #ty }
    });

    let ix_struct = if ix.args.is_empty() {
        quote! {
            pub struct #ix_struct_name;
        }
    } else {
        quote! {
            pub struct #ix_struct_name {
                #(#fields),*
            }
        }
    };

    let discriminator = gen_discriminator(&ix.discriminator);
    let impl_discriminator = quote! {
        impl anchor_lang::Discriminator for #ix_struct_name {
            const DISCRIMINATOR: &'static [u8] = &#discriminator;
        }
    };

    let impl_ix_data = quote! {
        impl anchor_lang::InstructionData for #ix_struct_name {}
    };

    // let program_id = get_canonical_program_id();
    // let impl_owner = quote! {
    //     impl anchor_lang::Owner for #ix_struct_name {
    //         fn owner() -> Pubkey {
    //             #program_id
    //         }
    //     }
    // };

    quote! {
        /// Instruction argument
        #[derive(anchor_lang::AnchorSerialize, anchor_lang::AnchorDeserialize)]
        #ix_struct

        #impl_discriminator
        #impl_ix_data
    }
    // #impl_owner
}

/// Generate types used by all instructions
/// TODO: Slim down types based on which instructions are used, and recurse any sub-types
fn gen_types_mod(idl: &Idl) -> proc_macro2::TokenStream {
    let types = idl
        .types
        .iter()
        .filter(|ty| {
            // Skip accounts and events
            !(idl.accounts.iter().any(|acc| acc.name == ty.name)
                || idl.events.iter().any(|ev| ev.name == ty.name))
        })
        .map(|ty| convert_idl_type_def_to_ts(ty, &idl.types));

    quote! {
        /// Program type definitions.
        ///
        /// Note that account and event type definitions are not included in this module, as they
        /// have their own dedicated modules.
        pub mod types {
            use super::*;

            #(#types)*
        }
    }
}

fn gen_actions_enum(input: &DeclareAnchorActions) -> TokenStream {
    let action_mod_name = format_ident!("{}_actions", input.name.to_string().to_snake_case());
    let enum_name = format_ident!("{}Action", input.name.to_string().to_upper_camel_case());

    let mut variants = Vec::with_capacity(input.instructions.len());

    for ix in &input.instructions {
        let ix_name = format_ident!("{}", ix.name.to_string().to_upper_camel_case());
        let ix_struct_name = format_ident!("{}", ix.name.to_string().to_upper_camel_case());

        let variant: TokenStream = quote! {
            #ix_name(#action_mod_name::#ix_struct_name),
        };

        variants.push(variant);
    }

    quote! {
        pub enum #enum_name {
            #(#variants)*
        }
    }
}

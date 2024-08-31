use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    ItemTrait, Token,
};

#[derive(Debug)]
pub struct ActionEnumInput {
    enum_name: Ident,
    trait_definition: ItemTrait,
    variants: Punctuated<Ident, Token![,]>,
}

impl Parse for ActionEnumInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let trait_definition: ItemTrait = input.parse()?;
        input.parse::<Token![,]>()?;
        let variants = Punctuated::parse_terminated(input)?;

        Ok(ActionEnumInput {
            enum_name,
            trait_definition,
            variants,
        })
    }
}

pub fn parse_define_action_enum(input: ActionEnumInput) -> TokenStream {
    let ActionEnumInput {
        enum_name,
        trait_definition,
        variants,
    } = input;

    let trait_name = &trait_definition.ident;
    let trait_methods = &trait_definition.items;

    // Define variant names for enum
    let variant_defs = variants.iter().map(|variant| {
        quote! {
            #variant(#variant),
        }
    });

    // Generate `From` implementations for each variant
    let from_impls = variants.iter().map(|variant| {
        quote! {
            impl From<#variant> for #enum_name {
                fn from(inner: #variant) -> Self {
                    #enum_name::#variant(inner)
                }
            }
        }
    });

    // Generate match arms for each trait method dynamically
    let method_impls = trait_methods.iter().filter_map(|item| {
        if let syn::TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let match_arms = variants.iter().map(|variant| {
                quote! {
                    #enum_name::#variant(inner) => inner.#method_name(),
                }
            });

            // Implement the trait method
            Some(quote! {
                fn #method_name(&self) -> bool {
                    match self {
                        #(#match_arms)*
                    }
                }
            })
        } else {
            None
        }
    });

    let expanded = quote! {
        #trait_definition

        // Define the action enum
        #[macros::action_enum]
        pub enum #enum_name {
            #(#variant_defs)*
        }

        // Map variants to correct trait impls
        impl #trait_name for #enum_name {
            #(#method_impls)*
        }

        // Implement `From` for each variant
        #(#from_impls)*
    };

    expanded
}

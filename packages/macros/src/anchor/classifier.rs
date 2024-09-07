use anchor_lang_idl::types::Idl;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Ident, Token,
};

use crate::anchor::util::gen_id;

use super::util::get_idl;

pub struct AnchorClassifierInput {
    name: syn::Ident,
    idl: Idl,
    variants: Punctuated<Ident, Comma>,
}

impl Parse for AnchorClassifierInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let idl = get_idl(&name).map_err(|e| syn::Error::new(name.span(), e))?;
        input.parse::<Token![,]>()?;

        let variants = Punctuated::parse_terminated(input)?;

        Ok(Self {
            name,
            idl,
            variants,
        })
    }
}

pub fn gen_anchor_classifier(input: AnchorClassifierInput) -> TokenStream {
    let classifier_name =
        format_ident!("{}Classifier", input.name.to_string().to_upper_camel_case());

    let errors = gen_errors(&input);
    let classifier_impl = gen_classifier_impl(&input);

    quote! {
        pub struct #classifier_name;

        #errors

        #classifier_impl
    }
}

fn gen_errors(input: &AnchorClassifierInput) -> TokenStream {
    let error_ident = format_ident!("{}Error", input.name.to_string().to_upper_camel_case());

    quote! {
        #[derive(Debug, thiserror::Error)]
        pub enum #error_ident {
            #[error("Invalid instruction data length")]
            InvalidLength,
            #[error("Failed to deserialize instruction")]
            DeserializationError(#[source] anyhow::Error),
        }
    }
}

fn gen_classifier_impl(input: &AnchorClassifierInput) -> TokenStream {
    let anchor_mod = format_ident!("{}_anchor", input.name.to_string().to_snake_case());
    let actions_mod = format_ident!("{}_actions", input.name.to_string().to_snake_case());

    let classifier_name =
        format_ident!("{}Classifier", input.name.to_string().to_upper_camel_case());

    let enum_name = format_ident!("{}Action", input.name.to_string().to_upper_camel_case());

    let id = gen_id(&input.idl);

    let mut arms = Vec::with_capacity(input.variants.len());

    for variant in &input.variants {
        let arm = quote! {
            if ix.data.starts_with(actions::#anchor_mod::internal::#variant::DISCRIMINATOR) {
                let decoded = actions::#actions_mod::#variant::from_instruction(txn, ix)?;
                return Ok(Some(actions::#enum_name::#variant(decoded.into()).into()))
            }
        };

        arms.push(arm);
    }

    quote! {
        impl classifier_trait::InstructionClassifier for #classifier_name {
            #id

            fn classify_instruction(
                txn: &classifier_core::ClassifiableTransaction,
                ix: &classifier_core::ClassifiableInstruction) -> classifier_trait::ClassifyInstructionResult {

                #(#arms)*

                Ok(None)
            }
        }
    }
}

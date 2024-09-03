use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemEnum, ItemStruct};

pub fn get_action_derivations() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    }
}

pub fn parse_action_attribute(input: ItemStruct) -> TokenStream {
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    let struct_fields = &input.fields;

    let derivations = get_action_derivations();
    
    // Generate the derive statement with the common traits
    let expanded = quote! {
        #derivations
        #struct_vis struct #struct_name #struct_fields
    };

    expanded
}

pub fn parse_action_attribute_enum(input: ItemEnum) -> TokenStream {
    let enum_name = &input.ident;
    let enum_vis = &input.vis;
    let enum_variants = &input.variants;
    let enum_generics = &input.generics;

    let derivations = get_action_derivations();

    // Generate the derive statement with the common traits
    let expanded = quote! {
        #derivations
        #enum_vis enum #enum_name #enum_generics {
            #enum_variants
        }
    };

    expanded
}

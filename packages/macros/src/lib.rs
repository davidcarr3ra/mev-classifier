use action::{
    action_enum::{parse_define_action_enum, ActionEnumInput},
    attribute::{parse_action_attribute, parse_action_attribute_enum},
};
use anchor::{gen_anchor_action_declarations, gen_anchor_classifier, AnchorClassifierInput, DeclareAnchorActions};
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum, ItemStruct};

pub(crate) mod action;
pub(crate) mod anchor;

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    parse_action_attribute(input).into()
}

#[proc_macro_attribute]
pub fn action_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);
    parse_action_attribute_enum(input).into()
}

#[proc_macro]
pub fn define_actions(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ActionEnumInput);
    parse_define_action_enum(input).into()
}

#[proc_macro]
pub fn declare_anchor_actions(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeclareAnchorActions);
    gen_anchor_action_declarations(input).into()
}

#[proc_macro]
pub fn declare_anchor_classifier(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AnchorClassifierInput);
    gen_anchor_classifier(input).into()
}

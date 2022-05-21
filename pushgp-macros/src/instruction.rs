use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    bracketed, parse_macro_input, Attribute, Error, Ident, ItemFn, Meta, NestedMeta, PathSegment,
    Token,
};

struct InstructionStruct {
    attributes: Vec<Attribute>,
    fn_token: Token![fn],
    name: Ident,
}

pub fn handle_instruction_macro(inner_fn: &mut ItemFn) -> Result<TokenStream> {
    let stack_name = format!("{}", get_stack_name(&inner_fn)?);
    let new_fn_name = format!("{}_{}", stack_name.to_case(Case::Snake), inner_fn.sig.ident);
    inner_fn.sig.ident = syn::parse_str::<Ident>(&new_fn_name)?;

    // Only keep the 'doc' attributes
    inner_fn.attrs.retain(|attr| attr.path.is_ident("doc"));

    Ok(quote! { #inner_fn }.into())
}

fn get_stack_name(inner_fn: &ItemFn) -> Result<Ident> {
    // Get the token from the `#[stack(MyStackType)]` attribute
    if let Some(attr) = inner_fn
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("stack"))
        .next()
    {
        match attr.parse_meta() {
            Ok(Meta::List(list)) => {
                if list.nested.len() != 1 {
                    return Err(syn::Error::new(
                        list.span(),
                        "the `#[stack(MyType)]` attribute must have exactly one type",
                    ));
                } else {
                    match list.nested.first().unwrap() {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            if let Some(ident) = path.get_ident() {
                                return Ok(ident.clone());
                            } else {
                                return Err(syn::Error::new(path.span(), "the `#[stack(MyType)]` attribute must have a type that is a single Ident"));
                            }
                        }
                        _ => {
                            return Err(syn::Error::new(
                                list.span(),
                                "the `#[stack(MyType)]` attribute must have exactly one type",
                            ))
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Err(syn::Error::new(
        inner_fn.span(),
        "the function must have a `#[stack(MyType)]` attribute",
    ))
}

extern crate quote;

use crate::{item_fn::ItemFn, requirement_list::RequirementList};
use proc_macro::TokenStream;
use quote::*;
use syn::parse_macro_input;

mod instruction;
mod instruction_list;
mod item_fn;
mod requirement_list;
mod signature;
mod stack_instruction;

#[proc_macro]
pub fn instruction_list(input: TokenStream) -> TokenStream {
    instruction_list::make_instruction_list(input).into()
}

#[proc_macro]
pub fn instruction(input: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(input as ItemFn);
    instruction::handle_instruction_macro(&mut item_fn)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn stack_instruction(attr: TokenStream, input: TokenStream) -> TokenStream {
    let stack_ident = parse_macro_input!(attr as RequirementList);
    let mut item_fn = parse_macro_input!(input as ItemFn);
    stack_instruction::handle_macro(&stack_ident, &mut item_fn)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Display)]
#[doc(hidden)]
pub fn display(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        syn::Data::Enum(ref enum_data) => {
            let name = &ast.ident;
            impl_display(name, enum_data).into()
        }
        _ => panic!("#[derive(Display)] works only on enums"),
    }
}

fn impl_display(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data
        .variants
        .iter()
        .map(|variant| impl_display_for_variant(name, variant));

    quote! {
        impl Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                match *self {
                    #(#variants)*
                }
            }
        }
    }
}

fn impl_display_for_variant(name: &syn::Ident, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let id = &variant.ident;
    let upper = syn::Ident::new(&format!("{}", id).to_uppercase(), id.span());
    match variant.fields {
        syn::Fields::Unit => {
            quote! {
                #name::#id => {
                    f.write_str(stringify!(#upper))
                }
            }
        }
        _ => panic!("#[derive(Display)] works only with unit variants"),
    }
}

#[proc_macro_derive(ExecuteInstruction)]
#[doc(hidden)]
pub fn execute_instruction(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        syn::Data::Enum(ref enum_data) => {
            let name = &ast.ident;
            impl_execute_instruction(name, enum_data).into()
        }
        _ => panic!("#[derive(ExecuteInstruction)] works only on enums"),
    }
}

fn impl_execute_instruction(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let variants = data
        .variants
        .iter()
        .map(|variant| impl_execute_instruction_for_variant(name, variant));

    quote! {
        pub fn execute_instruction(context: &mut Context, instruction: Instruction) {
            match instruction {
                #(#variants)*
            }
        }
    }
}

fn impl_execute_instruction_for_variant(
    name: &syn::Ident,
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    let id = &variant.ident;
    let lower = syn::Ident::new(&format!("execute_{}", id).to_lowercase(), id.span());
    match variant.fields {
        syn::Fields::Unit => {
            quote! {
                #name::#id => #lower(context),
            }
        }
        _ => panic!("#[derive(ExecuteInstruction)] works only with unit variants"),
    }
}

#[proc_macro_derive(NomTag)]
#[doc(hidden)]
pub fn nom_tag(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        syn::Data::Enum(ref enum_data) => {
            let name = &ast.ident;
            impl_nom_tag(name, enum_data).into()
        }
        _ => panic!("#[derive(NomTag)] works only on enums"),
    }
}

fn impl_nom_tag(name: &syn::Ident, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let mut tags = vec![];
    for v in data.variants.iter() {
        tags.push(impl_nom_tag_for_variant(v));
        if tags.len() == 21 {
            let sub_alt = quote! {
                alt(
                    (#(#tags)*)
                ),
            };
            tags = vec![sub_alt];
        }
    }
    let text_to_instructions = data
        .variants
        .iter()
        .map(|variant| impl_nom_text_to_instruction_for_variant(name, variant));

    quote! {
        impl NomTag for #name {
            fn nom_tag(input: &str) -> IResult<&str, Instruction> {
                let (input, tag_text) = alt(
                    (#(#tags)*)
                )(input)?;
                let (input, _) = space0(input)?;

                Ok((input, match tag_text {
                    #(#text_to_instructions)*
                    _ => panic!("should never get here")
                }))
            }
        }
    }
}

fn impl_nom_tag_for_variant(variant: &syn::Variant) -> proc_macro2::TokenStream {
    let id = &variant.ident;
    let upper = syn::Ident::new(&format!("{}", id).to_uppercase(), id.span());
    match variant.fields {
        syn::Fields::Unit => {
            quote! {
                tag(stringify!(#upper)),
            }
        }
        _ => panic!("#[derive(NomTag)] works only with unit variants"),
    }
}

fn impl_nom_text_to_instruction_for_variant(
    name: &syn::Ident,
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    let id = &variant.ident;
    let upper = syn::Ident::new(&format!("{}", id).to_uppercase(), id.span());
    match variant.fields {
        syn::Fields::Unit => {
            quote! {
                stringify!(#upper) => #name::#id,
            }
        }
        _ => panic!("#[derive(NomTag)] works only with unit variants"),
    }
}

#[proc_macro_derive(ConfigureAllInstructions)]
#[doc(hidden)]
pub fn configure_all_instructions(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        syn::Data::Enum(ref enum_data) => {
            let name = &ast.ident;
            impl_configure_all_instructions(name, enum_data).into()
        }
        _ => panic!("#[derive(ConfigureAllInstructions)] works only on enums"),
    }
}

fn impl_configure_all_instructions(
    name: &syn::Ident,
    data: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let variants = data
        .variants
        .iter()
        .map(|variant| impl_configure_all_instructions_for_variant(name, variant));

    quote! {
        impl ConfigureAllInstructions for #name {
            fn configure_all_instructions(config: &mut Configuration, default_weight: u8) {
                #(#variants)*
            }
        }
    }
}

fn impl_configure_all_instructions_for_variant(
    name: &syn::Ident,
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    let id = &variant.ident;
    match variant.fields {
        syn::Fields::Unit => {
            quote! {
                config.set_instruction_weight(#name::#id, default_weight);
            }
        }
        _ => panic!("#[derive(ConfigureAllInstructions)] works only with unit variants"),
    }
}

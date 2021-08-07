use proc_macro::TokenStream;
use quote::*;

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

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Error, FnArg, Ident, ItemFn, Meta, NestedMeta, Pat, Path, Type};

struct FunctionParseResults {
    // The Pascal case name of every stack we have detected.
    pub stacks: HashSet<String>,

    // The argument names of the values that we should pop of their stacks, organized by stack. The first value in each
    // Vec will be the first value popped.
    pub pop: HashMap<String, Vec<Ident>>,
}

pub fn handle_instruction_macro(inner_fn: &mut ItemFn) -> Result<TokenStream> {
    let mut parse_results = FunctionParseResults {
        stacks: HashSet::default(),
        pop: HashMap::default(),
    };

    // Determine the full path that we should reference the 'pushgp' library in our code
    let pushgp =
        match crate_name("pushgp").map_err(|e| Error::new(inner_fn.span(), e.to_string()))? {
            FoundCrate::Itself => "crate".to_owned(),
            FoundCrate::Name(path) => path,
        };
    let pushgp: Path = syn::parse_str::<Path>(&pushgp)?;

    // Read the #[stack(MyType)] attribute to determine the base stack name
    let stack_name = format!("{}", get_stack_name(&inner_fn)?).to_case(Case::Pascal);
    parse_results.stacks.insert(stack_name.clone());

    // Use the base stack name plus the name of the function to generate the name of the struct
    let function_name = inner_fn.sig.ident.to_string();
    let struct_name: Ident = syn::parse_str::<Ident>(&format!(
        "{}{}",
        stack_name.to_case(Case::Pascal),
        function_name.to_case(Case::Pascal)
    ))?;

    // Use the base stack name plus the name of the function to generate the name of the instruction
    let instruction_name_str = format!(
        "{}.{}",
        stack_name.to_case(Case::UpperFlat),
        function_name.to_case(Case::UpperFlat)
    );

    // Only keep the 'doc' attributes from what's supplied for the function
    inner_fn.attrs.retain(|attr| attr.path.is_ident("doc"));
    let docs = inner_fn.attrs.iter();

    // Parse the fn arguments
    parse_arguments(&inner_fn, &mut parse_results)?;

    Ok(quote! {
        #(#docs)*
        pub struct #struct_name<C, L> {
            c: std::marker::PhantomData<C>,
            l: std::marker::PhantomData<L>,
        }
        impl<C, L> #pushgp::InstructionTrait<C> for #struct_name<C, L>
        where
            C: #pushgp::Context + ContextHasBoolStack<L>,
            L: #pushgp::LiteralEnum<L>,
        {
            fn name() -> &'static str {
                #instruction_name_str
            }

            fn execute(context: &mut C) {
            }
        }
    }
    .into())
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
                    return Err(Error::new(
                        list.span(),
                        "the `#[stack(MyType)]` attribute must have exactly one type",
                    ));
                } else {
                    match list.nested.first().unwrap() {
                        NestedMeta::Meta(Meta::Path(path)) => {
                            if let Some(ident) = path.get_ident() {
                                return Ok(ident.clone());
                            } else {
                                return Err(Error::new(path.span(), "the `#[stack(MyType)]` attribute must have a type that is a single Ident"));
                            }
                        }
                        _ => {
                            return Err(Error::new(
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

    Err(Error::new(
        inner_fn.span(),
        "the function must have a `#[stack(MyType)]` attribute",
    ))
}

fn parse_arguments(inner_fn: &ItemFn, parse_results: &mut FunctionParseResults) -> Result<()> {
    let mut has_context = false;
    for fn_arg in inner_fn.sig.inputs.iter() {
        if !has_context {
            match fn_arg {
                FnArg::Typed(pat_type) => {
                    // Check the name of the first parameter
                    if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                        if pat_ident.ident != "context" {
                            return Err(Error::new(fn_arg.span(), "the function's first parameter must be `context: &mut Context`"));
                        }
                        // otherwise, the name of this parameter is okay
                    } else {
                        return Err(Error::new(fn_arg.span(), "the function's first parameter must be `context: &mut Context`"));
                    }

                    // The type must be &mut Context
                    if let Type::Reference(type_ref) = pat_type.ty.as_ref() {
                        if type_ref.mutability.is_none() {
                            return Err(Error::new(type_ref.span(), "the function's first parameter must be `context: &mut Context`"));
                        }
                        if let Type::Path(type_path) = type_ref.elem.as_ref() {
                            if !type_path.path.is_ident("Context") {
                                return Err(Error::new(type_path.span(), "the function's first parameter must be `context: &mut Context`"));
                            }
                        }
                    } else {
                        return Err(Error::new(fn_arg.span(), "the function's first parameter must be `context: &mut Context`"));
                    }
                }
                _ => return Err(Error::new(fn_arg.span(), "the function's first parameter must be `context: &mut Context`"))
            }
            has_context = true;
        } else {

        }
    }

    if !has_context {
        return Err(Error::new(inner_fn.span(), "the function must have at least one parameter of `context: &mut Context`"));
    }
    Ok(())
}
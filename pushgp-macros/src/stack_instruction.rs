use crate::item_fn::ItemFn;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::parse::Result;
use syn::spanned::Spanned;
use syn::{Block, Error, Expr, FnArg, Ident, Meta, NestedMeta, Pat, Path, Stmt, Type};

struct FunctionParseResults {
    // The Pascal case name of every stack we have detected.
    pub stacks: HashSet<String>,

    // The argument names of the values that we should pop of their stacks, organized by stack. The first value in each
    // Vec will be the first value popped.
    pub pop: HashMap<String, Vec<Ident>>,
}

pub fn handle_macro(stack_ident: &Ident, inner_fn: &mut ItemFn) -> Result<TokenStream> {
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

    // Determine the base stack name
    let stack_name = format!("{}", stack_ident).to_case(Case::Pascal);
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

    Ok(quote! {
        pub struct #struct_name {}

        impl StaticName for #struct_name {
            fn static_name() -> &'static str {
                #instruction_name_str
            }
        }
    })

    // Ok(quote! {
    //     #(#docs)*
    //     pub struct #struct_name {}
    //     impl #pushgp::Instruction for #struct_name {

    //         fn name() -> &'static str {
    //             #instruction_name_str
    //         }

    //         fn execute<State: std::fmt::Debug + Clone>(context: &#pushgp::Context<State>, data: Option<#pushgp::InstructionData>) #body
    //     }
    // }
    // .into())
}
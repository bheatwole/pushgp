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
        #[derive(Debug, PartialEq)]
        pub struct #struct_name {}

        impl StaticName for #struct_name {
            fn static_name() -> &'static str {
                #instruction_name_str
            }
        }
        
        impl<Vm: VirtualMachine + VirtualMachineMustHaveBool<Vm>> StaticInstruction<Vm> for #struct_name {
            fn parse(input: &str) -> nom::IResult<&str, #pushgp::Code<Vm>> {
                let (rest, _) = nom::bytes::complete::tag(#struct_name::static_name())(input)?;
                let (rest, _) = crate::parse::space_or_end(rest)?;

                Ok((rest, Box::new(#struct_name {})))
            }

            fn random_value(_vm: &mut Vm) -> #pushgp::Code<Vm> {
                Box::new(#struct_name {})
            }
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(#struct_name::static_name())
            }
        }

        impl<Vm: VirtualMachineMustHaveBool<Vm>> Instruction<Vm> for #struct_name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        
            fn name(&self) -> &'static str {
                #struct_name::static_name()
            }
        
            fn clone(&self) -> #pushgp::Code<Vm> {
                Box::new(#struct_name{})
            }
        
            #(#docs)*
            fn execute(&mut self, vm: &mut Vm) {
                // if vm.bool().len() >= 2 {
                //     let a = vm.bool().pop().unwrap();
                //     let b = vm.bool().pop().unwrap();
                //     vm.bool().push(a && b);
                // }
            }
        
        }
    })
}
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{bracketed, parse_macro_input, Error, Ident, PathSegment, Token};

pub struct ContextName {
    pub name: Ident,
}

impl Parse for ContextName {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "context_name" {
            return Err(Error::new(
                key.span(),
                "The first field must be 'context_name'",
            ));
        }
        input.parse::<Token![:]>()?;

        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        Ok(ContextName { name })
    }
}

pub struct LiteralName {
    pub name: Ident,
}

impl Parse for LiteralName {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "literal_name" {
            return Err(Error::new(
                key.span(),
                "The second field must be 'literal_name'",
            ));
        }
        input.parse::<Token![:]>()?;

        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        Ok(LiteralName { name })
    }
}

pub struct LiteralList {
    pub literal_names: Punctuated<Ident, Token![,]>,
}

impl Parse for LiteralList {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "literals" {
            return Err(Error::new(key.span(), "The third field must be 'literals'"));
        }
        input.parse::<Token![:]>()?;

        let content;
        bracketed!(content in input);
        let literal_names = content.parse_terminated(Ident::parse)?;
        input.parse::<Token![,]>()?;

        Ok(LiteralList { literal_names })
    }
}

pub struct StackList {
    pub stack_names: Punctuated<Ident, Token![,]>,
}

impl Parse for StackList {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "stacks" {
            return Err(Error::new(key.span(), "The fourth field must be 'stacks'"));
        }
        input.parse::<Token![:]>()?;

        let content;
        bracketed!(content in input);
        let stack_names = content.parse_terminated(Ident::parse)?;
        input.parse::<Token![,]>()?;

        Ok(StackList { stack_names })
    }
}

pub struct InstructionList {
    pub instructions: Punctuated<Instruction, Token![,]>,
}

impl Parse for InstructionList {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "instructions" {
            return Err(Error::new(
                key.span(),
                "The fourth field must be 'instructions'",
            ));
        }
        input.parse::<Token![:]>()?;

        let content;
        bracketed!(content in input);
        let instructions = content.parse_terminated(Instruction::parse)?;
        input.parse::<Token![,]>()?;

        Ok(InstructionList { instructions })
    }
}

pub struct Instruction {
    pub instruction: Punctuated<PathSegment, Token![::]>,
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Instruction {
            instruction: Punctuated::<PathSegment, Token![::]>::parse_separated_nonempty(input)?,
        })
    }
}

pub struct Wrapper {
    pub context_name: ContextName,
    pub literal_name: LiteralName,
    pub literals: LiteralList,
    pub stacks: StackList,
    pub instructions: InstructionList,
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let context_name: ContextName = input.parse()?;
        let literal_name: LiteralName = input.parse()?;
        let literals: LiteralList = input.parse()?;
        let stacks: StackList = input.parse()?;
        let instructions: InstructionList = input.parse()?;

        Ok(Wrapper {
            context_name,
            literal_name,
            literals,
            stacks,
            instructions,
        })
    }
}

pub fn make_instruction_list(tokens: TokenStream) -> TokenStream {
    let parse = parse_macro_input!(tokens as Wrapper);
    let enum_definition = make_literal_enum_definition(&parse);
    let display = make_display_for_literal_enum(&parse);
    let impl_literal_enum = make_impl_for_literal_enum(&parse);
    let has_literal_value_impls = make_has_literal_value_for_literal_enum(&parse);
    let impl_ephemeral_configuration = make_impl_ephemeral_configuration_for_literal_enum(&parse);

    let context_name = parse.context_name.name;
    let literal_name = parse.literal_name.name;
    let get_all_instructions = parse.instructions.instructions.iter().map(|inst| {
        let path = &inst.instruction;
        quote! {
            #path::<#context_name, #literal_name>::name().to_owned()
        }
    });
    let parser_name: Ident = syn::parse_str(&format!("{}Parser", literal_name)).unwrap();
    let mut all_tags: Vec<proc_macro2::TokenStream> = parse.instructions.instructions.iter().map(|inst| {
        let path = &inst.instruction;
        quote! {
            tag(#path::<#context_name, #literal_name>::name())
        }
    }).collect();
    all_tags = make_nom_alt_tree(all_tags);
    let all_context_has = parse.stacks.stack_names.iter().map(|s| {
        let context_name: Ident = syn::parse_str(&format!("ContextHas{}Stack", s)).unwrap();
        quote!(#context_name<L>)
    });
    let all_add_to_table = parse.instructions.instructions.iter().map(|inst| {
        let path = &inst.instruction;
        quote! {
            #path::<C, L>::add_to_table(&mut instructions);
        }
    });

    quote! {
        #enum_definition
        #display
        #impl_literal_enum
        #(#has_literal_value_impls)*
        #impl_ephemeral_configuration

        impl InstructionConfiguration for #literal_name {
            fn get_all_instructions() -> Vec<String> {
                vec![
                    #(#get_all_instructions),*
                ]
            }
        }

        pub struct #parser_name {}
        impl Parser<#literal_name> for #parser_name {
            fn parse_code_instruction(input: &str) -> IResult<&str, Code<#literal_name>> {
                use nom::{branch::alt, bytes::complete::tag};
                let (input, instruction) = alt((
                    #(#all_tags),*
                ))(input)?;
                let (input, _) = crate::parse::space_or_end(input)?;

                Ok((input, Code::Instruction(instruction.to_owned())))
            }
        }
        
        pub fn new_instruction_table_with_all_instructions<C, L>() -> InstructionTable<C>
        where
            C: Context + #(#all_context_has)+*,
            L: LiteralEnum<L>,
        {
            let mut instructions = InstructionTable::new();
            #(#all_add_to_table)*

            instructions
        }
    }
    .into()
}

/// In the `nom` crate, the `alt` list can only hold 21 items and must hold at least two items. However, it can hold
/// `alt` sub-lists allowing us to make a tree of alt(alt(20), alt(2))
fn make_nom_alt_tree(mut all_tags: Vec<proc_macro2::TokenStream>) -> Vec<proc_macro2::TokenStream> {
    while all_tags.len() > 21 {
        // Determine what grouping we need for the tags because we can't have a remainder of 1.
        let mut grouping = 21;
        while 1 == all_tags.len() % grouping {
            grouping -= 1;
        }

        let mut grouped_tags = vec![];
        for chunk in all_tags.chunks(grouping) {
            grouped_tags.push(quote! {
                alt((#(#chunk),*))
            });
        }
        std::mem::swap(&mut grouped_tags, &mut all_tags);
    }

    all_tags
}

fn make_literal_enum_definition(parse: &Wrapper) -> proc_macro2::TokenStream {
    let all_enum_values: Vec<proc_macro2::TokenStream> = parse.literals.literal_names.iter().map(|name| {
        quote! {
            #name(#name)
        }
    }).collect();

    quote! {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub enum BaseLiteral {
            #(#all_enum_values),*
        }
    }
}

fn make_display_for_literal_enum(parse: &Wrapper) -> proc_macro2::TokenStream {
    let literal_name = &parse.literal_name.name;
    let all_enum_values: Vec<proc_macro2::TokenStream> = parse.literals.literal_names.iter().map(|name| {
        quote! {
            #literal_name::#name(v) => v.nom_fmt(f)
        }
    }).collect();

    quote! {
        impl std::fmt::Display for #literal_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self {
                    #(#all_enum_values),*
                }
            }
        }
    }
}

fn make_impl_for_literal_enum(parse: &Wrapper) -> proc_macro2::TokenStream {
    let literal_name = &parse.literal_name.name;
    let all_enum_values: Vec<proc_macro2::TokenStream> = parse.literals.literal_names.iter().map(|name| {
        quote! {
            if let Ok((rest, value)) = #name::parse(input) {
                return Ok((rest, #literal_name::#name(value)));
            }
        }
    }).collect();

    quote! {
        impl LiteralEnum<#literal_name> for #literal_name {
            fn parse(input: &str) -> IResult<&str, #literal_name> {
                #(#all_enum_values)*
        
                Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Complete)))
            }
        }
    }
}

fn make_has_literal_value_for_literal_enum(parse: &Wrapper) -> Vec<proc_macro2::TokenStream> {
    let literal_name = &parse.literal_name.name;

    parse.literals.literal_names.iter().map(|name| {
        quote! {
            impl LiteralEnumHasLiteralValue<#literal_name, #name> for #literal_name {
                fn supports_literal_type() -> bool {
                    true
                }
            
                fn make_from_value(value: #name) -> #literal_name {
                    #literal_name::#name(value)
                }
            }
        }
    }).collect()
}

fn make_impl_ephemeral_configuration_for_literal_enum(parse: &Wrapper) -> proc_macro2::TokenStream {
    let literal_name = &parse.literal_name.name;
    let all_enum_names: Vec<proc_macro2::TokenStream> = parse.literals.literal_names.iter().map(|name| {
        let name_quoted: syn::Lit = syn::parse_str(&format!("\"{}\"", quote!(#name).to_string())).unwrap();
        quote! {
            #name_quoted.to_owned()
        }
    }).collect();
    let all_enum_values: Vec<proc_macro2::TokenStream> = parse.literals.literal_names.iter().map(|name| {
        let name_quoted: syn::Lit = syn::parse_str(&format!("\"{}\"", quote!(#name).to_string())).unwrap();
        quote! {
            #name_quoted => LiteralConstructor { 0: |rng| #literal_name::#name(#name::random_value(rng)) },
        }
    }).collect();

    quote! {
        
        impl EphemeralConfiguration<#literal_name> for #literal_name {
            fn get_all_literal_types() -> Vec<String> {
                vec![
                    #(#all_enum_names),*
                ]
            }
        
            fn make_literal_constructor_for_type(literal_type: &str) -> LiteralConstructor<#literal_name> {
                match literal_type {
                    #(#all_enum_values)*
                    _ => panic!("unknown literal type"),
                }
            }
        }

    }
}
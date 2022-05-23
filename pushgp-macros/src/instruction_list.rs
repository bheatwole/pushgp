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

pub struct StackList {
    pub stack_names: Punctuated<Ident, Token![,]>,
}

impl Parse for StackList {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        if key != "stacks" {
            return Err(Error::new(key.span(), "The third field must be 'stacks'"));
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
    pub stacks: StackList,
    pub instructions: InstructionList,
}

impl Parse for Wrapper {
    fn parse(input: ParseStream) -> Result<Self> {
        let context_name: ContextName = input.parse()?;
        let literal_name: LiteralName = input.parse()?;
        let stacks: StackList = input.parse()?;
        let instructions: InstructionList = input.parse()?;

        Ok(Wrapper {
            context_name,
            literal_name,
            stacks,
            instructions,
        })
    }
}

pub fn make_instruction_list(tokens: TokenStream) -> TokenStream {
    let parse = parse_macro_input!(tokens as Wrapper);
    let context_name = parse.context_name.name;
    let literal_name = parse.literal_name.name;
    let get_all_instructions = parse.instructions.instructions.iter().map(|inst| {
        let path = &inst.instruction;
        quote! {
            #path::<#context_name, #literal_name>::name().to_owned()
        }
    });
    let parser_name: Ident = syn::parse_str(&format!("{}Parser", literal_name)).unwrap();
    let all_tags = parse.instructions.instructions.iter().map(|inst| {
        let path = &inst.instruction;
        quote! {
            tag(#path::<#context_name, #literal_name>::name())
        }
    });
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
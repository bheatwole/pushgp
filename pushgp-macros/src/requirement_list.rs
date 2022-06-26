use syn::parse::{Parse, ParseStream};
use syn::*;

pub struct RequirementList {
    pub idents: Vec<Ident>,
}

impl Parse for RequirementList {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut idents = vec![];
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            idents.push(ident);

            if input.is_empty() {
                break;
            }
            let _comma: Token![,] = input.parse()?;
        }

        Ok(RequirementList { idents })
    }
}

use crate::signature::Signature;
use syn::parse::{Parse, ParseStream};
use syn::{braced, Attribute, Block, Result, Visibility};
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};

/// We need to include our implementation of syn::Signature in the ItemFn instead of the default. This code is all
/// copied from the syn crate. Only the type of `sig` is different and line 31
pub struct ItemFn {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub block: Box<Block>,
}

impl Parse for ItemFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let outer_attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        parse_rest_of_fn(input, outer_attrs, vis, sig)
    }
}

fn parse_rest_of_fn(
    input: ParseStream,
    mut attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
) -> Result<ItemFn> {
    let content;
    let brace_token = braced!(content in input);
    attrs.extend_from_slice(&Attribute::parse_inner(&content)?); // slight change to use a public function
    let stmts = content.call(Block::parse_within)?;

    Ok(ItemFn {
        attrs,
        vis,
        sig,
        block: Box::new(Block { brace_token, stmts }),
    })
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.vis.to_tokens(tokens);
        self.sig.to_tokens(tokens);
        self.block.brace_token.surround(tokens, |tokens| {
            tokens.append_all(&self.block.stmts);
        });
    }
}
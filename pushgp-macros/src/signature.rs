use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::ToTokens;
use std::iter::FromIterator;
use std::mem;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{Pair, Punctuated};
use syn::token::{Async, Comma, Const, Fn, Paren, Unsafe};
use syn::*;

/// This is a slight re-implementation of syn::Signature, which only accepts an Ident for the name of the function. In
/// the instruction! macro we need to accept both Ident and a keyword.
pub struct Signature {
    pub constness: Option<Const>,
    pub asyncness: Option<Async>,
    pub unsafety: Option<Unsafe>,
    pub abi: Option<Abi>,
    pub fn_token: Fn,
    pub ident: Ident,
    pub generics: Generics,
    pub paren_token: Paren,
    pub inputs: Punctuated<FnArg, Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}

impl Parse for Signature {
    fn parse(input: ParseStream) -> Result<Self> {
        let constness: Option<Token![const]> = input.parse()?;
        let asyncness: Option<Token![async]> = input.parse()?;
        let unsafety: Option<Token![unsafe]> = input.parse()?;
        let abi: Option<Abi> = input.parse()?;
        let fn_token: Token![fn] = input.parse()?;
        let ident: Ident = input.call(Ident::parse_any)?; // This is the only change from the syn crate
        let mut generics: Generics = input.parse()?;

        let content;
        let paren_token = parenthesized!(content in input);
        let mut inputs = parse_fn_args(&content)?;
        let variadic = pop_variadic(&mut inputs);

        let output: ReturnType = input.parse()?;
        generics.where_clause = input.parse()?;

        Ok(Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        })
    }
}

fn parse_fn_args(input: ParseStream) -> Result<Punctuated<FnArg, Token![,]>> {
    let mut args = Punctuated::new();
    let mut has_receiver = false;

    while !input.is_empty() {
        let attrs = input.call(Attribute::parse_outer)?;

        let arg = if let Some(dots) = input.parse::<Option<Token![...]>>()? {
            FnArg::Typed(PatType {
                attrs,
                pat: Box::new(Pat::Verbatim(variadic_to_tokens(&dots))),
                colon_token: Token![:](dots.spans[0]),
                ty: Box::new(Type::Verbatim(variadic_to_tokens(&dots))),
            })
        } else {
            let mut arg: FnArg = input.parse()?;
            match &mut arg {
                FnArg::Receiver(receiver) if has_receiver => {
                    return Err(Error::new(
                        receiver.self_token.span,
                        "unexpected second method receiver",
                    ));
                }
                FnArg::Receiver(receiver) if !args.is_empty() => {
                    return Err(Error::new(
                        receiver.self_token.span,
                        "unexpected method receiver",
                    ));
                }
                FnArg::Receiver(receiver) => {
                    has_receiver = true;
                    receiver.attrs = attrs;
                }
                FnArg::Typed(arg) => arg.attrs = attrs,
            }
            arg
        };
        args.push_value(arg);

        if input.is_empty() {
            break;
        }

        let comma: Token![,] = input.parse()?;
        args.push_punct(comma);
    }

    Ok(args)
}

fn pop_variadic(args: &mut Punctuated<FnArg, Token![,]>) -> Option<Variadic> {
    let trailing_punct = args.trailing_punct();

    let last = match args.last_mut()? {
        FnArg::Typed(last) => last,
        _ => return None,
    };

    let ty = match last.ty.as_ref() {
        Type::Verbatim(ty) => ty,
        _ => return None,
    };

    let mut variadic = Variadic {
        attrs: Vec::new(),
        dots: parse2(ty.clone()).ok()?,
    };

    if let Pat::Verbatim(pat) = last.pat.as_ref() {
        if pat.to_string() == "..." && !trailing_punct {
            variadic.attrs = mem::replace(&mut last.attrs, Vec::new());
            args.pop();
        }
    }

    Some(variadic)
}

fn variadic_to_tokens(dots: &Token![...]) -> TokenStream {
    TokenStream::from_iter(vec![
        TokenTree::Punct({
            let mut dot = Punct::new('.', Spacing::Joint);
            dot.set_span(dots.spans[0]);
            dot
        }),
        TokenTree::Punct({
            let mut dot = Punct::new('.', Spacing::Joint);
            dot.set_span(dots.spans[1]);
            dot
        }),
        TokenTree::Punct({
            let mut dot = Punct::new('.', Spacing::Alone);
            dot.set_span(dots.spans[2]);
            dot
        }),
    ])
}

impl ToTokens for Signature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.constness.to_tokens(tokens);
        self.asyncness.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            let mut last_is_variadic = false;
            for input in self.inputs.pairs() {
                match input {
                    Pair::Punctuated(input, comma) => {
                        maybe_variadic_to_tokens(input, tokens);
                        comma.to_tokens(tokens);
                    }
                    Pair::End(input) => {
                        last_is_variadic = maybe_variadic_to_tokens(input, tokens);
                    }
                }
            }
            if self.variadic.is_some() && !last_is_variadic {
                if !self.inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.variadic.to_tokens(tokens);
            }
        });
        self.output.to_tokens(tokens);
        self.generics.where_clause.to_tokens(tokens);
    }
}

fn maybe_variadic_to_tokens(arg: &FnArg, tokens: &mut TokenStream) -> bool {
    let arg = match arg {
        FnArg::Typed(arg) => arg,
        FnArg::Receiver(receiver) => {
            receiver.to_tokens(tokens);
            return false;
        }
    };

    match arg.ty.as_ref() {
        Type::Verbatim(ty) if ty.to_string() == "..." => {
            match arg.pat.as_ref() {
                Pat::Verbatim(pat) if pat.to_string() == "..." => {
                    pat.to_tokens(tokens);
                }
                _ => arg.to_tokens(tokens),
            }
            true
        }
        _ => {
            arg.to_tokens(tokens);
            false
        }
    }
}

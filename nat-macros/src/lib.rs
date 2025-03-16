use syn::{
    LitInt, Token, Type, parenthesized, parse::Parse, parse_macro_input, punctuated::Punctuated,
    token::Paren,
};

use quote::quote;

#[proc_macro]
pub fn nat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as LitInt);

    // Extract the value of the input integer
    let n = input.base10_parse::<u64>().unwrap();

    let mut result = quote! { tlist::Zero };

    for _ in 0..n {
        result = quote! { tlist::Succ<#result> };
    }

    result.into()
}

struct ExprAdd {
    #[allow(dead_code)]
    add: Token![+],
    exprs: Punctuated<Expr, Token![,]>,
}

struct ExprMul {
    #[allow(dead_code)]
    mul: Token![*],
    exprs: Punctuated<Expr, Token![,]>,
}

struct ExprParen {
    #[allow(dead_code)]
    paren: Paren,
    expr: Box<Expr>,
}

enum Expr {
    Type(Type),
    Add(ExprAdd),
    Mul(ExprMul),
    Paren(ExprParen),
}

impl Parse for ExprAdd {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            add: input.parse()?,
            exprs: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Parse for ExprMul {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mul: input.parse()?,
            exprs: Punctuated::parse_terminated(input)?,
        })
    }
}

impl Parse for ExprParen {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in input),
            expr: Box::new(content.parse()?),
        })
    }
}

impl Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![+]) {
            Ok(Self::Add(input.parse()?))
        } else if input.peek(Token![*]) {
            Ok(Self::Mul(input.parse()?))
        } else if input.peek(Paren) {
            Ok(Self::Paren(input.parse()?))
        } else {
            Ok(Self::Type(input.parse()?))
        }
    }
}

trait Eval {
    fn eval(&self) -> Result<proc_macro2::TokenStream, syn::Error>;
}

impl Eval for ExprAdd {
    fn eval(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        let evals: Vec<_> = self
            .exprs
            .iter()
            .map(|expr| expr.eval())
            .collect::<Result<_, _>>()?;

        Ok(evals.into_iter().fold(quote! { tlist::Zero }, |acc, eval| {
            quote! { <#acc as tlist::Nat>::Add<#eval> }
        }))
    }
}

impl Eval for ExprMul {
    fn eval(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        let evals: Vec<_> = self
            .exprs
            .iter()
            .map(|expr| expr.eval())
            .collect::<Result<_, _>>()?;

        Ok(evals.into_iter().fold(quote! { tlist::Succ<tlist::Zero> }, |acc, eval| {
            quote! { <#acc as tlist::Nat>::Mul<#eval> }
        }))
    }
}

impl Eval for ExprParen {
    fn eval(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        self.expr.eval()
    }
}

impl Eval for Expr {
    fn eval(&self) -> Result<proc_macro2::TokenStream, syn::Error> {
        match self {
            Self::Type(ty) => Ok(quote! { #ty }),
            Self::Add(add) => add.eval(),
            Self::Mul(mul) => mul.eval(),
            Self::Paren(paren) => paren.eval(),
        }
    }
}

#[proc_macro]
pub fn expr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let result = input.eval();
    match result {
        Ok(result) => result.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

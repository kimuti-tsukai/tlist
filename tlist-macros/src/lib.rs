use std::iter;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Abi, AngleBracketedGenericArguments, FnArg, GenericArgument, Generics, Ident, ImplItemFn,
    ImplItemType, ItemFn, ItemImpl, LitStr, PatTuple, PatType, Path, PathArguments, PathSegment,
    QSelf, Receiver, ReturnType, Signature, Token, Type, TypePath, TypeTuple, Visibility,
    punctuated::Punctuated, token::SelfType,
};

#[proc_macro_attribute]
pub fn hkt(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);

    let fn_name = input.sig.ident;

    let struct_name = TypePath {
        qself: None,
        path: fn_name.clone().into(),
    };

    let generics = input.sig.generics;

    let (patterns, arg_types): (Punctuated<_, Token![,]>, Punctuated<_, Token![,]>) = input
        .sig
        .inputs
        .iter()
        .cloned()
        .filter_map(|arg| {
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                Some((*pat, *ty))
            } else {
                None
            }
        })
        .unzip();

    let arg_type = TypeTuple {
        paren_token: Default::default(),
        elems: arg_types,
    };

    let fn_arg = PatType {
        attrs: Vec::new(),
        pat: Box::new(
            PatTuple {
                attrs: Vec::new(),
                paren_token: Default::default(),
                elems: patterns,
            }
            .into(),
        ),
        colon_token: Default::default(),
        ty: Box::new(arg_type.clone().into()),
    };

    let output_type = match &input.sig.output {
        ReturnType::Default => TypeTuple {
            paren_token: Default::default(),
            elems: Punctuated::new(),
        }
        .into(),
        ReturnType::Type(_, ty) => *ty.clone(),
    };

    let body = input.block;

    let fn_traits = ["FnOnce", "FnMut", "Fn"]
        .into_iter()
        .map(|i| Ident::new(i, Span::call_site()))
        .map(|ident| PathSegment {
            ident,
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: [GenericArgument::Type(arg_type.clone().into())]
                    .into_iter()
                    .collect(),
                gt_token: Default::default(),
            }),
        });

    let output_type = ImplItemType {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        type_token: Default::default(),
        generics: Generics::default(),
        ident: Ident::new("Output", Span::call_site()),
        eq_token: Default::default(),
        ty: output_type,
        semi_token: Default::default(),
    };

    let output_types = std::iter::once(Some(output_type)).chain(std::iter::repeat(None));

    let methods = ["call_once", "call_mut", "call"]
        .into_iter()
        .map(|s| Ident::new(s, Span::call_site()));

    let receivers = [
        Receiver {
            attrs: Vec::new(),
            reference: None,
            mutability: None,
            self_token: Default::default(),
            colon_token: None,
            ty: Box::new(
                TypePath {
                    qself: None,
                    path: SelfType::default().into(),
                }
                .into(),
            ),
        },
        Receiver {
            attrs: Vec::new(),
            reference: Some((Default::default(), None)),
            mutability: Some(Default::default()),
            self_token: Default::default(),
            colon_token: None,
            ty: Box::new(Type::Verbatim(TokenStream::new())),
        },
        Receiver {
            attrs: Vec::new(),
            reference: Some((Default::default(), None)),
            mutability: None,
            self_token: Default::default(),
            colon_token: None,
            ty: Box::new(Type::Verbatim(TokenStream::new())),
        },
    ];

    let methods = methods.zip(receivers).map(|(method, receiver)| ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: Some(Abi {
                extern_token: Default::default(),
                name: Some(LitStr::new("rust-call", Span::call_site())),
            }),
            fn_token: Default::default(),
            ident: method,
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: [FnArg::from(receiver), fn_arg.clone().into()]
                .into_iter()
                .collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(
                    TypePath {
                        qself: Some(QSelf {
                            lt_token: Default::default(),
                            ty: Box::new(
                                TypePath {
                                    qself: None,
                                    path: SelfType::default().into(),
                                }
                                .into(),
                            ),
                            position: 1,
                            as_token: Default::default(),
                            gt_token: Default::default(),
                        }),
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident::new("FnOnce", Span::call_site()),
                                    arguments: PathArguments::AngleBracketed(
                                        AngleBracketedGenericArguments {
                                            colon2_token: None,
                                            lt_token: Default::default(),
                                            args: [GenericArgument::Type(arg_type.clone().into())]
                                                .into_iter()
                                                .collect(),
                                            gt_token: Default::default(),
                                        },
                                    ),
                                },
                                Ident::new("Output", Span::call_site()).into(),
                            ]
                            .into_iter()
                            .collect(),
                        },
                    }
                    .into(),
                ),
            ),
        },
        block: *body.clone(),
    });

    let impls =
        fn_traits
            .zip(output_types)
            .zip(methods)
            .map(|((fn_trait, output_type), method)| ItemImpl {
                attrs: Vec::new(),
                defaultness: None,
                unsafety: None,
                impl_token: Default::default(),
                generics: generics.clone(),
                trait_: Some((None, fn_trait.into(), Default::default())),
                self_ty: Box::new(struct_name.clone().into()),
                brace_token: Default::default(),
                items: {
                    output_type
                        .map(Into::into)
                        .into_iter()
                        .chain(iter::once(method.into()))
                        .collect()
                },
            });

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        struct #fn_name;

        #(#impls)*
    };

    expanded.into()
}

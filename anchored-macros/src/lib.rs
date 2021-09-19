#![feature(proc_macro_quote)]

#[macro_use]
extern crate proc_macro_error;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, FnArg, Generics, Ident, ItemFn, Pat, Token, TypeParamBound, WhereClause,
    WherePredicate,
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn unanchored(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = TokenStream::new();
    result.extend(attr);
    result.extend(item.clone());

    let future = parse_macro_input!(item as ItemFn);

    if future.sig.asyncness.is_none() {
        let msg = "this isn't an async function";
        return syn::Error::new_spanned(future.sig.ident, msg)
            .to_compile_error()
            .into();
    }

    let syn::ItemFn {
        attrs: _attrs,
        vis: _vis,
        block: _block,
        sig,
    } = future;

    let syn::Signature {
        inputs,
        unsafety,
        abi,
        ident,
        generics:
            Generics {
                params: generic_param,
                where_clause,
                ..
            },
        ..
    } = sig;

    let unanchored_ident = format_ident!("__assert_unanchored_{}", ident);

    let func_params = extract_params(&inputs);
    let where_clause = rewrite_where(where_clause);

    let assert_fn: TokenStream = quote!(
        #unsafety #abi fn #unanchored_ident<#generic_param> (#inputs) #where_clause{
            let future = #ident(#(#func_params),*);
            fn assert<UnanchoredFutureType: anchored::Unanchored>(_: UnanchoredFutureType) {}
            assert(future);
        }
    )
    .into();

    result.extend(assert_fn);
    result
}

/// Extract parameter `Ident`s from `FnArg`.
///
/// E.g.,
/// ```rust, no_run
/// fn(ident_1: Type1, ident_2: Type2)
/// ```
/// returns
/// ```rust, no_run
/// vec![Ident("ident_1"), Ident("ident_2")]
/// ```
fn extract_params(inputs: &Punctuated<FnArg, Comma>) -> Vec<Ident> {
    inputs
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                    return pat_ident.ident.clone();
                }
            }
            panic!()
        })
        .collect()
}

/// Add `Unanchored` to type params in "where" clause.
///
/// E.g.,
/// ```rust, no_run
/// -> where T: Sync,
/// ```
/// to
/// ```rust, no_run
/// -> where T: Sync + Unanchored,
/// ```
fn rewrite_where(where_clause: Option<WhereClause>) -> Option<WhereClause> {
    if let Some(mut where_clause) = where_clause {
        let span = where_clause.span();
        where_clause.predicates = where_clause
            .predicates
            .into_iter()
            .map(|pred| match pred {
                WherePredicate::Lifetime(lifetime) => WherePredicate::Lifetime(lifetime),
                WherePredicate::Eq(eq) => WherePredicate::Eq(eq),
                WherePredicate::Type(mut ty) => {
                    ty.bounds.push_punct(Token![+](span));
                    ty.bounds
                        .push(syn::parse_str::<TypeParamBound>("anchored::Unanchored").unwrap());

                    WherePredicate::Type(ty)
                }
            })
            .collect();

        Some(where_clause)
    } else {
        None
    }
}

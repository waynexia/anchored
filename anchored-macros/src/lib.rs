#![feature(proc_macro_quote)]

#[macro_use]
extern crate proc_macro_error;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat};

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
        generics: _generics,
        ..
    } = sig;

    let unanchored_ident = format_ident!("__assert_unanchored_{}", ident);

    let params = extract_params(&inputs);

    let assert_fn: TokenStream = quote!(
        #unsafety #abi fn #unanchored_ident (#inputs){
            let future = #ident(#(#params),*);
            fn assert<T: Unanchored>(_: T) {}
            assert(future);
        }
    )
    .into();

    result.extend(assert_fn);
    result
}

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

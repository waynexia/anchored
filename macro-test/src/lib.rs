#![allow(dead_code)]
use ::anchored::unanchored;

#[unanchored]
async fn simple() {}

#[unanchored]
async fn with_return_value() -> usize {
    42
}

#[unanchored]
async fn with_param(input: usize) -> usize {
    input
}

#[unanchored]
#[allow(clippy::needless_lifetimes)]
async fn with_lifetime<'a>(input: &'a usize) -> &'a usize {
    input
}

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
async fn with_lifetime<'a, 'b>(input: &'a usize) -> &'b usize
where
    'a: 'b,
{
    input
}

#[unanchored]
async fn with_type_param<T>(input: T) -> T
where
    T: Send + Sync + 'static,
{
    input
}

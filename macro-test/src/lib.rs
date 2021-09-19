#![allow(dead_code)]
use std::sync::Arc;

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

struct AsyncMethod {}

impl AsyncMethod {
    #[unanchored]
    async fn by_ref(&self, input: usize) -> usize {
        input
    }

    #[unanchored]
    async fn by_ref_mut(&mut self, input: usize) -> usize {
        input
    }

    #[unanchored]
    async fn by_attr_ref_mut(#[allow(unused_mut)] mut self: Arc<Self>, input: usize) -> usize {
        input
    }

    #[unanchored]
    async fn by_arc(self: Arc<Self>, input: usize) -> usize {
        input
    }

    #[unanchored]
    async fn by_arc_mut(self: &mut Arc<Self>, input: usize) -> usize {
        input
    }

    #[unanchored]
    async fn by_mut_arc(#[allow(unused_mut)] mut self: Arc<Self>, input: usize) -> usize {
        input
    }
}

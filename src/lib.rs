#![feature(negative_impls)]
#![feature(auto_traits)]

mod anchored;

pub use crate::anchored::{Anchored, Unanchored};
pub use anchored_macros::unanchored;

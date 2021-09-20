//! Make things anchored and forbid them from crossing `.await` point.
//!
//! # Quick Example
//! The two basic things in this crate are:
//! - [unanchored] attribute macro for marking functions and methods you want to apply the `.await` point crossing check.
//! - [Anchored] wrapper struct to wrap over things you want to anchored.
//!
//! They usually work with each other like this: **First** wrap [Anchored] on the object you want it to keep away from `.await`,
//! like `MutexGuard`, a `&mut` reference or anything else. **Then** add [unanchored] attribute to the async function / method to
//! enable this compile time check.
//!
//! That's all. Now the compiler will check it for you.
//!
//! For example, the following code can't compile because `bar` is trying to cross the `.await` point.
//! ```rust, ignore
//! # use anchored::{Anchored, unanchored};
//! # struct Bar{}
//! # async fn async_fn(){}
//! #[unanchored]
//! async fn foo(){
//!     let bar = Anchored::new(Bar {});
//!     async_fn().await;
//!     drop(bar);
//! }
//! ```
//! And after limiting `bar`'s scope, everything is fine.
//! ```rust
//! # use anchored::{Anchored, unanchored};
//! # struct Bar{}
//! # async fn async_fn(){}
//! #[unanchored]
//! async fn foo(){
//!     {
//!         let bar = Anchored::new(Bar {});
//!     }
//!     async_fn().await;
//! }
//! ```
//!
//! # Motivation
//! Some type is not intended to be used under async context, like the blocking `Mutex` from std, or interior mutable wrapper
//! `RefCell` or `UnsafeCell`. Keeping them across the `.await` point may cause unexpected problems like data race, dead lock etc.
//! When using these types in an async block, we need to check it carefully to ensure they are bounded in the sync context. In other
//! words, not crossing the `.await` point.
//!
//! This crate provides a way to enforce this check at compile time. Providing more safety than manually check and can avoid some checks
//! at runtime like `RefCell` does.
//!
//! # How
//! This crate is pretty simple. It brings an auto trait `Unanchored`, and opt-out it for [Anchored]. Like the `Unpin` trait and `Pin` wrapper couple from std.
//!
//! We use the mechanism that when converting an async block
//! into generator, all the variables in scope before one `.await` will be captured into generator's state. By checking the converted future's
//! type we can tell whether an [Anchored] is captured (crossed the `.await` point).
//!
//! # Limitation
//! The type inference phase only cares about the "scope" and won't take "ownership" into consideration. Which means drop the variable does works fine
//! and have no problem in runtime, but it will fail the check:
//! ```rust, ignore
//! # use anchored::{Anchored, unanchored};
//! # struct Bar{}
//! # async fn async_fn(){}
//! #[unanchored]
//! async fn foo(){
//!     let bar = Anchored::new(Bar {});
//!     drop(bar);
//!     async_fn().await;
//! }
//! ```
//! It should be enclosed into a smaller scope like above example explicit.
//!
//! # Related Work
//! [clippy][clippy] provides two lint options `await_holding_lock` and `await_holding_refcell_ref` that can check some specific lock and reference types.
//!
//! [clippy]: https://github.com/rust-lang/rust-clippy

#![feature(negative_impls)]
#![feature(auto_traits)]

mod anchored;

pub use crate::anchored::{Anchored, Unanchored};
pub use anchored_macros::unanchored;

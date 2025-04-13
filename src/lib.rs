//! The [ChainLink](https://docs.rs/hypotaxis/0.1.0/hypotaxis/trait.ChainLink.html)
//! trait with a blanket implementation gives access to methods that can help to
//! create longer method chains.
//! 
//! ```rust
//! use hypotaxis::ChainLink;
//! ```

#![no_std]
mod apply;
mod conditional;
mod mutated;
mod tap;

use apply::apply;
use conditional::*;
use mutated::mutated;
use tap::tap;

pub trait ChainLink: Sized {
    r#if!{}
    if_else!{}
    if_eval!{}
    if_eval_else!{}
    apply!{}
    tap!{}
    mutated!{}
}

impl<T> ChainLink for T {}
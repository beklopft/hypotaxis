mod apply;
mod conditional;
mod mutated;
mod tap;

use apply::apply;
use conditional::*;
use mutated::mutated;
use tap::tap;

pub trait Changeable: Sized {
    r#if!{}
    if_else!{}
    if_eval!{}
    if_eval_else!{}
    apply!{}
    tap!{}
    mutated!{}
}

impl<T> Changeable for T {}
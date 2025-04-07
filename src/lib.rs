mod apply;
mod conditional;
mod mutated;
mod tap;

pub trait Changeable: Sized {
    r#if!{}
    if_else!{}
    if_eval!{}
    if_eval_else!{}
    apply!{}
    tap!{}
    mutated!{}
}

impl<T: 'static> Changeable for T {}
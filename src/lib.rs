mod apply;
mod conditional;
mod tap;

pub trait Changeable: Sized {
    r#if!{}
    if_else!{}
    apply!{}
    tap!{}
}

impl<T: 'static> Changeable for T {}
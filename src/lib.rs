pub trait Changeable: Sized {
    fn r#if<F>(
        self,
        condition: bool,
        change: F
    ) -> Self
    where
        F: FnOnce(Self) -> Self
    {
        if condition {
            change(self)
        } else {
            self
        }
    }
}

impl<T: 'static> Changeable for T {}
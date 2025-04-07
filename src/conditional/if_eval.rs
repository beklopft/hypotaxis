#[macro_export]
macro_rules! if_eval { () => {
    fn if_eval<C, F>(
        self,
        condition: C,
        change: F
    ) -> Self
    where
        C: FnOnce(&Self) -> bool,
        F: FnOnce(Self) -> Self
    {
        if condition(&self) {
            change(self)
        } else {
            self
        }
    }
}}
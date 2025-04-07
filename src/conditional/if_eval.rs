#[macro_export]
macro_rules! if_eval { () => {
    fn if_eval<P, F>(
        self,
        predicate: P,
        change: F
    ) -> Self
    where
        P: FnOnce(&Self) -> bool,
        F: FnOnce(Self) -> Self
    {
        if predicate(&self) {
            change(self)
        } else {
            self
        }
    }
}}
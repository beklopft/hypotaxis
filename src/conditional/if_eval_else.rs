macro_rules! if_eval_else { () => {
    fn if_eval_else<P, F, T>(
        self,
        predicate: P,
        change: F,
        default_change: F
    ) -> T
    where
        P: FnOnce(&Self) -> bool,
        F: FnOnce(Self) -> T
    {
        if predicate(&self) {
            change(self)
        } else {
            default_change(self)
        }
    }
}}

pub(crate) use if_eval_else;
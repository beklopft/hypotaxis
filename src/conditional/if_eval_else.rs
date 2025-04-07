#[macro_export]
macro_rules! if_eval_else { () => {
    fn if_eval_else<C, F, T>(
        self,
        condition: C,
        change: F,
        default_change: F
    ) -> T
    where
        C: FnOnce(&Self) -> bool,
        F: FnOnce(Self) -> T
    {
        if condition(&self) {
            change(self)
        } else {
            default_change(self)
        }
    }
}}
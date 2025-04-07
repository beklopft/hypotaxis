#[macro_export]
macro_rules! if_else { () => {
    fn if_else<F, T>(
        self,
        condition: bool,
        change: F,
        default_change: F
    ) -> T
    where F: FnOnce(Self) -> T
    {
        if condition {
            change(self)
        } else {
            default_change(self)
        }
    }
}}
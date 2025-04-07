#[macro_export]
macro_rules! apply { () => {
    fn apply<F, T>(
        self,
        change: F
    ) -> T
    where F: FnOnce(Self) -> T
    {
        change(self)
    }
}}
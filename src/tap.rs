#[macro_export]
macro_rules! tap { () => {
    //inspired by the tap crate
    fn tap<F>(
        self,
        func: F
    ) -> Self
    where F: FnOnce(&Self) -> Self
    {
        func(&self);

        self
    }
}}
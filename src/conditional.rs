#[macro_export]
macro_rules! r#if { () => {
    fn r#if<F>(
        self,
        condition: bool,
        change: F
    ) -> Self
    where F: FnOnce(Self) -> Self
    {
        if condition {
            change(self)
        } else {
            self
        }
    }
}}

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
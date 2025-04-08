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

pub(crate) use r#if;
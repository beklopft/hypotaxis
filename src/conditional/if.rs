macro_rules! r#if { () => {
    /// Applies the closure only if the condition is `true`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chainsmith::Changeable;
    /// use std::io::{stdout, IsTerminal};
    ///
    /// std::process::Command::new("ls")
    ///     .arg("-t")
    ///     .r#if( stdout().is_terminal(), |command| command
    ///         .arg("-l")
    ///     )
    ///     .arg("-l")
    ///     .spawn()
    ///     .unwrap()
    /// ;
    /// ```
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
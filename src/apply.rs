macro_rules! apply { () => {
    /// Applies the closure on the input value. This
    /// can give you a block inside the method chain.
    /// `apply()` is to a single type what
    /// [`map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
    /// is to
    /// every item of an terator.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use hypotaxis::ChainLink;
    /// 
    /// let appointment_day = 11;
    /// 
    /// let reminder_date = appointment_day
    ///     .apply(|day| day - 1)
    ///     .apply(|day| format!("2025.04.{day:#02}"))
    /// ;
    /// 
    /// assert_eq!(reminder_date, "2025.04.10");
    /// ```
    fn apply<F, T>(
        self,
        change: F
    ) -> T
    where F: FnOnce(Self) -> T
    {
        change(self)
    }
}}

pub(crate) use apply;
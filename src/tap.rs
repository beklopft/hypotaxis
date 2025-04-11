macro_rules! tap { () => {
    /// Gives access to the value inside a method chain.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use hypotaxis::Changeable;
    /// 
    /// let passcode = 33344850294026550922u128;
    /// 
    /// passcode
    ///     .tap(|passcode| {
    ///         #[cfg(debug_assertions)]
    ///         dbg!(*passcode);
    ///     })
    ///     .tap(|passcode| assert!(*passcode != 0))
    /// ;
    /// ```
    //from the tap crate
    fn tap<F>(
        self,
        func: F
    ) -> Self
    where F: FnOnce(&Self)
    {
        func(&self);

        self
    }
}}

pub(crate) use tap;
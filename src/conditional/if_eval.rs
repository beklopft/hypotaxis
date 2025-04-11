macro_rules! if_eval { () => {
    /// Applies the closure only if the predicate evaluates
    /// to `true`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use hypotaxis::ChainLink;
    ///
    /// let indexes = [1, 2, 3, 4, 5].to_vec();
    /// 
    /// let all_indexes = indexes
    ///     .if_eval(
    ///         |indexes| indexes[0] > 0,
    ///         |indexes| {
    ///             let mut indexes = indexes;
    ///
    ///             indexes.insert(0, 0);
    ///
    ///             indexes
    ///         }
    ///     )
    /// ;
    ///
    /// assert_eq!(all_indexes, [0, 1, 2, 3, 4, 5].to_vec())
    /// ```
    fn if_eval<P, F>(
        self,
        predicate: P,
        change: F
    ) -> Self
    where
        P: FnOnce(&Self) -> bool,
        F: FnOnce(Self) -> Self
    {
        if predicate(&self) {
            change(self)
        } else {
            self
        }
    }
}}

pub(crate) use if_eval;
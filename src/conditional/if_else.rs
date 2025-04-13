macro_rules! if_else { () => {
    /// Applies one closure if the condition is `true`,
    /// otherwise applies the other closure. Other than
    /// [`if()`](./trait.ChainLink.html#method.if) `if_else()` can change the return type.
    /// 
    /// ```
    /// use hypotaxis::ChainLink;
    /// 
    /// const ALWAYS_MORE: bool = true;
    ///
    /// let longer = [1, 2, 3]
    ///     .into_iter()
    ///     .if_else(
    ///         ALWAYS_MORE,
    ///         |iter| iter.chain(vec![4, 5, 6]),
    ///         |iter| iter.chain(vec![])
    ///     )
    ///     .collect::<Vec<_>>()
    /// ;
    ///
    /// assert_eq!(longer, [1, 2, 3, 4, 5, 6].to_vec())
    /// ```
    fn if_else<A, B, T>(
        self,
        condition: bool,
        change: A,
        default_change: B
    ) -> T
    where
        A: FnOnce(Self) -> T,
        B: FnOnce(Self) -> T
    {
        if condition {
            change(self)
        } else {
            default_change(self)
        }
    }
}}

pub(crate) use if_else;
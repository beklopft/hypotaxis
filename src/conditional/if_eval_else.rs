macro_rules! if_eval_else { () => {
    /// Applies one closure if the predicate
    /// evaluates to true, otherwise applies the
    /// other closure. Other than [`if_eval`]
    /// `if_eval_else` can change the return type.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let data = [2.26, 3.32, 3.78, 9.39, 7.51];
    ///
    /// let final_data = data
    /// .if_eval_else(
    ///     |data| data.len() < 10,
    ///     |data| data
    ///         .into_iter()
    ///         .chain([10.0, f64::INFINITY].to_vec())
    ///     ,
    ///     |data| data
    ///         .into_iter()
    ///         .chain([].to_vec())
    /// )
    /// .collect::<Vec<_>>()
    /// ;
    ///
    /// assert_eq!(
    ///     final_data,
    ///     [2.26, 3.32, 3.78, 9.39, 7.51, 10.0, f64::INFINITY].to_vec()
    /// )
    /// ```
    fn if_eval_else<P, A, B, T>(
        self,
        predicate: P,
        change: A,
        default_change: B
    ) -> T
    where
        P: FnOnce(&Self) -> bool,
        A: FnOnce(Self) -> T,
        B: FnOnce(Self) -> T
    {
        if predicate(&self) {
            change(self)
        } else {
            default_change(self)
        }
    }
}}

pub(crate) use if_eval_else;
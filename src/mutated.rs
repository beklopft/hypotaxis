macro_rules! mutated { () => {
    /// Returns the mutated value for further method
    /// calling.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use chainsmith::Changeable;
    /// 
    /// #[derive(Debug, PartialEq)]
    /// struct Person(String);
    /// 
    /// let names = ["Bob", "Alice", "Charlie"].to_vec();
    /// 
    /// let people_ordered = names
    ///     .mutated( |names| { names
    ///         .sort();
    ///     })
    ///     .into_iter()
    ///     .map(
    ///         |name| Person(name.to_string())
    ///     )
    ///     .collect::<Vec<_>>()
    /// ;
    /// 
    /// assert_eq!(people_ordered, [
    ///     Person("Alice".to_string()),
    ///     Person("Bob".to_string()),
    ///     Person("Charlie".to_string()),
    /// ])
    /// ```
    fn mutated<F>(
        self,
        mutate: F
    ) -> Self
    where F: FnOnce(&mut Self)
    {
        let mut value = self;
        mutate(&mut value);

        value
    }
}}

pub(crate) use mutated;
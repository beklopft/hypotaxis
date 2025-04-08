macro_rules! mutated { () => {
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
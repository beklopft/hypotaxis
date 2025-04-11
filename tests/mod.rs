use hypotaxis::ChainLink;

#[test]
fn change() {
    let value = 1i32;
    let too_small = true;

    let result = value.r#if(
        too_small,
        |value| value + 1
    );

    assert_eq!(result, 2);
}

#[test]
fn default() {
    let value = 1;
    let too_small = false;

    let result = value.r#if(
        too_small,
        |value| value + 1
    );

    assert_eq!(result, 1);
}

#[test]
fn vec() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let should_be_joined = true;

    let result = vec1.r#if(
        should_be_joined,
        |vec1| [vec1, vec2].concat()
    );

    assert_eq!(result, vec![1, 2, 3, 4, 5, 6])
}

#[cfg(not(debug_assertions))]
#[test]
fn iterators() {
    let x = [1, 2, 3].into_iter();

    struct Percentage(pub f64);
    impl Percentage {
        fn is_valid(&self) -> bool {
            (0.0..=1.0).contains(&self.0)
        }

        fn is_impossible(&self) -> bool {
            self.0 != 0.0
        }
    }

    const ACCEPT_IMPOSSIBLE: bool = false;

    let mut important_data = [0.803, 0.815, 0.151, 0.0, 0.783, 0.690, 0.632];

    let refined = important_data
        .into_iter()
        .map(|num| {
            let percentage = Percentage(num);
            assert!(percentage.is_valid());

            percentage
        })
        .if_else(
            !ACCEPT_IMPOSSIBLE, 
            |percentages| percentages
                .filter(|percentage| !percentage.is_impossible()),
            |percentages| percentages
                .filter(|x| true)
        )
        .collect::<Vec<_>>()
    ;
}

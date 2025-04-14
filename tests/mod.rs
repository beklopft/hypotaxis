use hypotaxis::ChainLink;

#[test]
fn readme_example_with_crate() {
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64
    }

    #[derive(Debug, PartialEq)]
    struct ProbabilityMap {
        data: Vec<(Point, f64)>
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
    }

    let raw_data: [(Point, f64); 4] = [
        (Point::new(8.23, -8.78), 0.803),
        (Point::new(-3.75, 3.33), 0.815),
        (Point::new(-0.71, -0.03), 0.151),
        (Point::new(-2.66, -2.66), 0.0),
    ];
    const NEW_DATA_VALID: bool = false;
    let new_raw_data = [
        (Point::new(-9.86, 8.18), -0.350),
        (Point::new(9.09, -8.62), 0.550),
    ];

    let finalized_data = raw_data
        .to_vec()
        .r#if(NEW_DATA_VALID, |original_data|
            [original_data, new_raw_data.to_vec()].concat()
        )
        .mutated(|data| data
            .sort_by(|a, b| a.1.total_cmp(&b.1))
        )
        .tap(|data| {
            assert!( data.iter()
                .all(|(_, probability)| (0.0..=1.0).contains(probability))
            )
        })
        .into_iter()
        .filter(|(_point, probability)| *probability > 0.0)
        .collect::<Vec<_>>()
        .apply(|data|
            ProbabilityMap { data }
        )
    ;

    assert_eq!(finalized_data, ProbabilityMap { data: [
        (Point::new(-0.71, -0.03), 0.151),
        (Point::new(8.23, -8.78), 0.803),
        (Point::new(-3.75, 3.33), 0.815),
    ].to_vec() })
}

#[test]
fn readme_example_without_crate() {
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64
    }

    #[derive(Debug, PartialEq)]
    struct ProbabilityMap {
        data: Vec<(Point, f64)>
    }

    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }
    }

    let raw_data: [(Point, f64); 4] = [
        (Point::new(8.23, -8.78), 0.803),
        (Point::new(-3.75, 3.33), 0.815),
        (Point::new(-0.71, -0.03), 0.151),
        (Point::new(-2.66, -2.66), 0.0),
    ];
    const NEW_DATA_VALID: bool = false;
    let new_raw_data: [(Point, f64); 2] = [
        (Point::new(-9.86, 8.18), -0.350),
        (Point::new(9.09, -8.62), 0.550),
    ];

    let full_raw_data = if NEW_DATA_VALID {
        [raw_data.to_vec(), new_raw_data.to_vec()].concat()
    } else {
        raw_data.to_vec()
    };

    let mut sorted = full_raw_data;
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1));

    assert!( sorted.iter()
        .all(|(_, probability)| (0.0..=1.0).contains(probability))
    );

    let finalized_data = ProbabilityMap {
        data: sorted
            .into_iter()
            .filter(|(_point, probability)| *probability > 0.0)
            .collect::<Vec<_>>()
    };

    assert_eq!(finalized_data, ProbabilityMap { data: [
        (Point::new(-0.71, -0.03), 0.151),
        (Point::new(8.23, -8.78), 0.803),
        (Point::new(-3.75, 3.33), 0.815),
    ].to_vec() })
}

#[test]
fn condition_true() {
    let value = 1i32;
    let too_small = true;

    let result = value.r#if(
        too_small,
        |value| value + 1
    );

    assert_eq!(result, 2);
}

#[test]
fn condition_false() {
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
use if_method::Changeable;

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

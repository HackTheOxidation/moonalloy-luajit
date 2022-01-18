use moonalloy::linalg::array::Array;

#[test]
fn test_array_dotp_on_itself() {
    let arr: Array = Array::from(&mut [1.0, 2.0, 3.0]);
    assert_eq!(arr.dotp(&arr), 14.0);
}

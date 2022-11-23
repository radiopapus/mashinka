mod common;

#[test]
fn integration_test() {
    common::setup();
    assert_eq!(4, 2 + 2);
}

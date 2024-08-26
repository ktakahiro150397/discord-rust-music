extern crate addr;

#[test]
fn integration_it_adds_two() {
    assert_eq!(4, addr::add_two(2));
}


#[test]
fn increment_int_using_references() {
    assert_eq!(references::increment_int_using_references(&mut 5), 6);
}

#[test]
fn increment_int_without_references() {
    let a = 5;
    
    assert_eq!(references::increment_int_without_references(a), 6);
}
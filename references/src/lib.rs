pub fn increment_int_using_references(a : &mut i32) -> i32 {
    let a = *a + 1;
    return a;
}

pub fn increment_int_without_references(a : i32) -> i32 {
    return a + 1;
}
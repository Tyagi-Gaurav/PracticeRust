#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() >= _second_list.len() {
        return determine_sublist(_first_list, _second_list, false);
    } else {
        return determine_sublist(_second_list, _first_list, true);
    }
}

fn determine_sublist<T: PartialEq>(
    _first_list: &[T],
    _second_list: &[T],
    is_reversed: bool,
) -> Comparison {
    let mut is_equal = true;
    for i in 0.._first_list.len() {
        if i + _second_list.len() <= _first_list.len() {
            let first_slice = &_first_list[i..i + _second_list.len()];
            let result = check_equality(first_slice, _second_list);

            if result == Comparison::Equal {
                if _first_list.len() == _second_list.len() {
                    return result;
                } else {
                    if is_reversed {
                        return Comparison::Sublist;
                    } else {
                        return Comparison::Superlist;
                    }
                }
            } else {
                is_equal = false;
            }
        } else {
            is_equal = false;
        }
    }

    if is_equal {
        return Comparison::Equal;
    } else {
        return Comparison::Unequal;
    }
}

fn check_equality<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //println!("Checking equality for lists of size: {}, {}", _first_list.len(), _second_list.len());
    for i in 0.._first_list.len() {
        if _first_list[i] != _second_list[i] {
            return Comparison::Unequal;
        }
    }

    return Comparison::Equal;
}



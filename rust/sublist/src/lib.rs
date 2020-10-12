#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T>(l1: &[T], l2: &[T]) -> Comparison
where
    T: PartialEq,
{
    if l1.len() < l2.len() && contains(l1, l2) {
        Comparison::Sublist
    } else if l1.len() > l2.len() && contains(l2, l1) {
        Comparison::Superlist
    } else if l1 == l2 {
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}

fn contains<T>(l1: &[T], l2: &[T]) -> bool
where
    T: PartialEq,
{
    l1.is_empty() || l2.windows(l1.len()).any(|sl| sl == l1)
}

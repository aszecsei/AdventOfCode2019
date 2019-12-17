mod point;
mod point3;

pub use point::*;
pub use point3::*;

pub fn replace_with<T>(list: &mut Vec<T>, replace: &[T], with: &[T]) -> bool
where
    T: PartialEq<T> + Clone,
{
    let first = list
        .as_slice()
        .windows(replace.len())
        .enumerate()
        .find(|&(_, s)| s == replace)
        .map(|(idx, _)| idx);
    if let Some(idx) = first {
        list.splice(idx..(idx + replace.len()), with.iter().cloned());
        return true;
    }
    false
}

pub fn replace_all_with<T>(list: &mut Vec<T>, replace: &[T], with: &[T])
where
    T: PartialEq<T> + Clone,
{
    while replace_with(list, replace, with) {}
}

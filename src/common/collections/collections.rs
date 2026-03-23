#![allow(dead_code)]

pub fn contains<T, F>(pred: F, col: &[T]) -> bool
where
    F: Fn(&T) -> bool,
{
    col.iter().any(pred)
}

// col: &[T]           = &[1, 2, 3, 4]   <- reference to the whole slice
// col.iter()          yields &T per element:
//                     &1, &2, &3, &4
// Self::Item          = &T               <- what the iterator yields (one element)
// find wraps it:      &&T per element
//                     &&1, &&2, &&3, &&4
// type Item = &'a T`
// 'a is the lifetime of the reference returned by find
// 'a T = reference to T with lifetime 'a
//

// pub fn some<I, F>(pred: F, mut col: I) -> Option<I::Item>
// where
//     I: Iterator,
//     F: Fn(&I::Item) -> bool,
// {
//     col.find(pred)
// }

pub fn when<I, F, P, R>(pred: P, mut col: I, action: F) -> Option<R>
where
    I: Iterator,
    P: Fn(&I::Item) -> bool,
    F: Fn(I::Item) -> R,
{
    let outcome = col.find(pred).map(|e| action(e));
    outcome
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn returns_first_match() {
//         assert_eq!(some(|x| x > &&2, vec![1, 2, 3, 4].iter()), Some(&3));
//     }

//     #[test]
//     fn returns_none_when_no_match() {
//         assert_eq!(some(|x| x > &&10, vec![1, 2, 3].iter()), None);
//     }

//     #[test]
//     fn empty_collection() {
//         assert_eq!(some(|x: &&i32| **x > 0, vec![].iter()), None);
//     }
// }

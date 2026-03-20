#![allow(dead_code)]

pub fn contains<T, F>(pred: F, col: &[T]) -> bool
where
    F: Fn(&T) -> bool,
{
    col.iter().any(pred)
}

pub fn some<T, F>(pred: F, col: &[T]) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
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
    col.iter().find(|x| pred(x))
}

pub fn when<T, F, P, R>(pred: P, col: &[T], action: F) -> Option<R>
where
    P: Fn(&T) -> bool,
    F: Fn(&T) -> R,
{
    let outcome = some(pred, col).map(|e| action(e));
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_match() {
        assert_eq!(some(|x| x > &2, &[1, 2, 3, 4]), Some(&3));
    }

    #[test]
    fn returns_none_when_no_match() {
        assert_eq!(some(|x| x > &10, &[1, 2, 3]), None);
    }

    #[test]
    fn empty_collection() {
        assert_eq!(some(|x: &i32| *x > 0, &[]), None);
    }
}

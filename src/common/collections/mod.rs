pub mod traits;

/*
 * given get_at(3)([a, b, c]) yields c
 * can be used to get the same element from multiple rows
 * useful for columnular indexing
 */
pub fn get_at<T: Clone>(index: usize) -> impl Fn(&Vec<T>) -> T {
    move |coll: &Vec<T>| coll[index].clone()
}

/* assume matrix = [[a, b, c], [d, e, f], [g, h, i]]
 * given get_column(1)(matrix) yields [b, e, h]
 */
pub fn get_column<T: Clone>(row_index: usize) -> impl Fn(&[Vec<T>]) -> Vec<T> {
    move |matrix: &[Vec<T>]| matrix.iter().map(get_at(row_index)).collect()
}

pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    let dimension = 0..matrix[0].len();
    let get_matrix_column = |vert_index| get_column(vert_index)(matrix);
    let transposed_matrix = dimension.map(get_matrix_column).collect();
    transposed_matrix
}

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
    let outcome = col.find(pred).map(action);
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            transpose(&matrix),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9],]
        );
    }

    #[test]
    fn single_row() {
        assert_eq!(transpose(&[vec![3, 7, 9]]), vec![vec![3], vec![7], vec![9]]);
    }

    #[test]
    fn rectangular() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            transpose(&matrix),
            vec![vec![1, 4], vec![2, 5], vec![3, 6],]
        );
    }

    #[test]
    fn contains_found() {
        assert!(contains(|x| *x > 3, &[1, 2, 3, 4]));
    }

    #[test]
    fn contains_not_found() {
        assert!(!contains(|x| *x > 10, &[1, 2, 3]));
    }

    #[test]
    fn when_match() {
        assert_eq!(when(|x| *x > &2, [1, 2, 3, 4].iter(), |x| x * 2), Some(6));
    }

    #[test]
    fn when_no_match() {
        assert_eq!(when(|x| *x > &10, [1, 2, 3].iter(), |x| x * 2), None);
    }
}

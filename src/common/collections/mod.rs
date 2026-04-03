pub mod traits;

pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    // range
    let range = 0..matrix[0].len();
    // [[3, 7, 9], [1, 2, 3]]
    let z = |horizontal_index| {
        // 1, 2, ...
        matrix
            .iter()
            .map(|horizontal_row| horizontal_row[horizontal_index].clone()) // [3, 7, 9][0] => [1, 2, 3][0]
            // [3, 7, 9] => 3 | 7 | 9
            .collect()
        //
    };
    let transposed_matrix = range.map(z).collect();
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
}

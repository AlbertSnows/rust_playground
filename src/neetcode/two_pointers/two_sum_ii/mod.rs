// # Two Sum II - Input Array Is Sorted

// Given a 1-indexed sorted array, return indices of two numbers that add up to target (use O(1) extra space).

// ## Example
// Input: numbers = [2,7,11,15], target = 9
// Output: [1,2]
use std::cmp::Ordering::{Equal, Greater, Less};

pub fn search_for_target(
    numbers: Vec<i32>,
    target: i32,
) -> impl FnMut((usize, usize), usize) -> Result<(usize, usize), (usize, usize)> {
    move |(l, r), _| {
        if l >= r {
            return Err((l, r)); // converged with no answer (shouldn't happen per problem)
        }
        let match_result = (numbers[l] + numbers[r]).cmp(&target);
        match match_result {
            Equal => Err((l, r)),      // found — short-circuit
            Less => Ok((l + 1, r)),    // sum too small, advance left
            Greater => Ok((l, r - 1)), // sum too big, retreat right
        }
    }
}

pub fn two_sum_ii(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // Both Ok (exhausted) and Err (short-circuited) carry the final state
    let result: Result<(usize, usize), (usize, usize)> =
        (0..numbers.len()).try_fold((0, numbers.len() - 1), search_for_target(numbers, target));
    let (l, r) = result.unwrap_or_else(|e| e);
    vec![l as i32 + 1, r as i32 + 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(two_sum_ii(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn adjacent() {
        assert_eq!(two_sum_ii(vec![2, 3, 4], 6), vec![1, 3]);
    }
}

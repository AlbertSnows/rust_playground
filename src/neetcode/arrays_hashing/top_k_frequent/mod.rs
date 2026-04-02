use crate::common::{
    collections::traits::AddIfNotExists,
    maps::{freq_map, hash_freq_map},
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
// Given an integer array and an integer k, return the k most frequent elements.

// ## Example
// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let num_freq = freq_map(nums.iter().copied()); // coppied gives me i32 instead of &i32, preventing &&i32 in the future.
    let freq_vec: Vec<(&i32, &usize)> = num_freq
        .iter()
        // type mismatch in closure arguments
        // expected closure signature `for<'a, 'b> fn(&'a (&&_, &_), &'b (&&i32, &_)) -> _`
        //    found closure signature `fn((&_, &_), (_, &_)) -> _`
        // `for<'a, 'b> fn(&'a (&&_, &_), &'b (&&i32, &_)) -> _`
        // `fn((&_, &_), (_, &_)) -> _`
        // --
        // for<'a, 'b> -> lifetime annotation, this means the references have lifetimes tied to the call.
        // means for any lifetimes, 'a and 'b are of the caller's choosing.The closure must work regardless of how long the references live.
        // generally, it indicates two parameters with separate lifetimes.
        //
        // fn(&'a (&&_, &_), &'b (&&i32, &_)) -> fn that expect two tuples (the types are unknown and outlined inside)
        //
        // fn((&_, &_), (_, &_)) -> fn that takes a tuple with references, the second one has no reference on the first value.
        // why &_, but _?
        // compiler didn't know the type of the first tuple, but figured it out by the second?
        .sorted_by(
            |(left_num, left_freq): &(&i32, &usize), (right_num, right_freq)| {
                left_freq.cmp(right_freq)
            },
        )
        .collect();
    let most_freq_k = freq_vec
        .into_iter()
        .take(k as usize)
        .map(|(num, _): (&i32, &usize)| *num)
        .collect::<Vec<i32>>();
    most_freq_k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut result = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn single_element() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }
}

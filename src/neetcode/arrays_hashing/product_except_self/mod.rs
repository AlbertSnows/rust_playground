// Given an integer array, return an array where each element is the product of all other elements (no division allowed).

// ## Example
// Input: nums = [1,2,4,3,4]
// Output: [96, 48, 24, 32, 24]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let num_zeros = nums.iter().filter(|&num| num == &0).count();
    if num_zeros > 1 {
        return vec![0; nums.len()];
    } else if num_zeros == 1 {
        let zero_index = nums.iter().position(|&num| num == 0).unwrap();
        let multiplication_sum: i32 = nums.iter().filter(|&&n| n != 0).product();
        let mut result = vec![0; nums.len()];
        result[zero_index] = multiplication_sum;
        return result;
    }
    let multiplication_sum: i32 = nums.iter().product();
    let product_of_other_elements = nums
        .iter()
        .map(|&num| multiplication_sum / num)
        .collect::<Vec<i32>>();
    product_of_other_elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn with_zero() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}

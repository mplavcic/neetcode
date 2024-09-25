/*
    MEDIUM

    Products of Array Discluding Self
    Given an integer array nums, return an array output where output[i] is the
    product of all the elements of nums except nums[i].Each product is guaranteed
    to fit in a 32-bit integer.
    Follow-up: Could you solve it in O(n) time without using the division operation?

    Example 1:
    Input: nums = [1,2,4,6]
    Output: [48,24,12,8]

    Example 2:
    Input: nums = [-1,0,1,2,3]
    Output: [0,-6,0,0,0]

    Constraints:
    2 <= nums.length <= 1000
    -20 <= nums[i] <= 20

*/

// time complexity: O(n^2)
// space complexity: O(n)
fn product_expect_self_naive(nums: &[i32]) -> Vec<i32> {
    let mut product_vec = Vec::with_capacity(nums.len());

    for i in 0..nums.len() {
        let mut product1 = 1;
        let mut product2 = 1;

        for num in &nums[..i] {
            product1 *= num;
        }

        for num in &nums[i + 1..] {
            product2 *= num;
        }

        product_vec.push(product1 * product2);
    }

    product_vec
}

// time complexity: O(n)
// space complexity: O(n)
fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut product_vec = vec![1; n];

    let mut prefix = 1;
    nums.iter().enumerate().for_each(|(i, num)| {
        product_vec[i] = prefix;
        prefix *= num;
    });

    let mut postfix = 1;
    nums.iter().enumerate().rev().for_each(|(i, num)| {
        product_vec[i] *= postfix;
        postfix *= num;
    });

    product_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_two_zeros() {
        let nums = [0, 0];
        assert_eq!(product_except_self(&nums), vec![0, 0]);
    }

    #[test]
    fn all_ones() {
        let nums = [1, 1, 1, 1];
        assert_eq!(product_except_self(&nums), vec![1, 1, 1, 1]);
    }

    #[test]
    fn one_element() {
        let nums = [5];
        assert_eq!(product_except_self(&nums), vec![1]);
    }

    #[test]
    fn all_negative() {
        let nums = [-2, -3, -4, -5];
        assert_eq!(product_except_self(&nums), vec![-60, -40, -30, -24]);
    }

    #[test]
    fn one_zero() {
        let nums = [2, 3, 0, 4];
        assert_eq!(product_except_self(&nums), vec![0, 0, 24, 0]);
    }

    #[test]
    fn positive_and_negative() {
        let nums = [10, -10, 2, -2];
        assert_eq!(product_except_self(&nums), vec![40, -40, 200, -200]);
    }
}

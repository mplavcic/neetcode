/*

    Two Integer Sum
    Given an array of integers nums and an integer target, return the indices i and j such
    that nums[i] + nums[j] == target and i != j.

    You may assume that every input has exactly one pair of indices i and j that satisfy the condition.

    Return the answer with the smaller index first.

    Example 1:
    Input: nums = [3,4,5,6], target = 7
    Output: [0,1]

    Example 2:
    Input: nums = [4,5,6], target = 10
    Output: [0,2]

    Example 3:
    Input: nums = [5,5], target = 10
    Output: [0,1]

    Constraints:
    2 <= nums.length <= 1000
    -10,000,000 <= nums[i] <= 10,000,000
    -10,000,000 <= target <= 10,000,000

*/

use std::collections::HashMap;

// Even though we assume that every input has exactly one pair of indices that satisfy the
// condition, we still return an Option since one of the constraints can be unsatisfied, in
// which case we return None. We are checking constraints in different if statements to allow
// appropriate logging. Another apporach is to return a Result where the Error variant holds
// info about any possible unsatisifed constraint.
//
pub fn two_sum_naive(nums: &[i64], target: i64) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }

    if nums.len() > 1000 {
        return None;
    }

    if target < -10000000 {
        return None;
    }

    if target > 10000000 {
        return None;
    }

    let mut first_index = 0;
    let mut second_index = 0;
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                first_index = i;
                second_index = j;
            }
        }
    }

    Some((first_index, second_index))
}

pub fn two_sum(nums: &[i64], target: i64) -> Option<(usize, usize)> {
    if nums.len() < 2 {
        return None;
    }

    if nums.len() > 1000 {
        return None;
    }

    if target < -10000000 {
        return None;
    }

    if target > 10000000 {
        return None;
    }

    // we assume there is exactly one combination so we chose the actual value as a key and index
    // as a value, since there should't be any overriding of values who are part of the combination
    let mut index_to_num: HashMap<i64, usize> = HashMap::new();
    let mut first_index: usize = 0;
    let mut second_index: usize = 0;

    for (index, num) in nums.iter().enumerate() {
        let complement = target - num;
        match index_to_num.get(&complement) {
            Some(complement_index) => {
                if &index < complement_index {
                    first_index = index;
                    second_index = *complement_index;
                } else {
                    first_index = *complement_index;
                    second_index = index;
                }
            }
            None => {
                index_to_num.insert(*num, index);
            }
        };
    }
    Some((first_index, second_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_naive() {
        let nums = [3, 4, 5, 6];
        let target = 7;

        assert_eq!(two_sum_naive(&nums, target), Some((0, 1)));
    }

    #[test]
    fn test2_naive() {
        let nums = [4, 5, 6];
        let target = 10;

        assert_eq!(two_sum_naive(&nums, target), Some((0, 2)));
    }

    #[test]
    fn test3_naive() {
        let nums = [5, 5];
        let target = 10;

        assert_eq!(two_sum_naive(&nums, target), Some((0, 1)));
    }
    #[test]
    fn test1() {
        let nums = [3, 4, 5, 6];
        let target = 7;

        assert_eq!(two_sum(&nums, target), Some((0, 1)));
    }

    #[test]
    fn test2() {
        let nums = [4, 5, 6];
        let target = 10;

        assert_eq!(two_sum(&nums, target), Some((0, 2)));
    }

    #[test]
    fn test3() {
        let nums = [5, 5];
        let target = 10;

        assert_eq!(two_sum(&nums, target), Some((0, 1)));
    }
}

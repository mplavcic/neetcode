/*

    MEDIUM

    Top K Elements in List
    Given an integer array nums and an integer k, return the k most frequent
    elements within the array. The test cases are generated such that the answer
    is always unique. You may return the output in any order.

    Example 1:
    Input: nums = [1,2,2,3,3,3], k = 2
    Output: [2,3]

    Example 2:
    Input: nums = [7,7], k = 1
    Output: [7]

    Constraints:
    1 <= nums.length <= 10^4.
    -1000 <= nums[i] <= 1000
    1 <= k <= number of distinct elements in nums.

*/

use std::collections::HashMap;

fn top_k_frequent(nums: &[u64], k: usize) -> Vec<u64> {
    let mut num_to_freq_map: HashMap<u64, i64> = HashMap::new();

    nums.iter().for_each(|num| {
        num_to_freq_map
            .entry(*num)
            .and_modify(|freq| *freq += 1)
            .or_insert(1);
    });

    let mut num_to_freq_vec: Vec<(u64, i64)> = num_to_freq_map.into_iter().collect();

    num_to_freq_vec.sort_by(|num, next_num| next_num.1.cmp(&num.1));

    num_to_freq_vec
        .into_iter()
        .take(k)
        .map(|(num, freq)| num)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = [1, 2, 2, 3, 3, 3];
        let k = 2;

        assert_eq!(top_k_frequent(&nums, k), vec![2, 3]);
    }

    #[test]
    fn test2() {
        let nums = [7, 7];
        let k = 1;

        assert_eq!(top_k_frequent(&nums, k), vec![7]);
    }
}

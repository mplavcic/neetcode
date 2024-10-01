/*

    MEDIUM

    Longest Consecutive Sequence
    Given an array of integers nums, return the length of the longest consecutive
    sequence of elements. A consecutive sequence is a sequence of elements in which
    each element is exactly 1 greater than the previous element.
    You must write an algorithm that runs in O(n) time.

    Example 1:
    Input: nums = [2,20,4,10,3,4,5]
    Output: 4
    Explanation: The longest consecutive sequence is [2, 3, 4, 5].

    Example 2:
    Input: nums = [0,3,2,5,4,6,1,1]
    Output: 7

    Constraints:
    0 <= nums.length <= 1000
    -10^9 <= nums[i] <= 10^9

*/

use std::collections::HashSet;

fn longest_consecutive(nums: &[i64]) -> u64 {
    let mut longest = 0;

    let seen: HashSet<i64> = nums.iter().cloned().collect();

    for num in seen.iter() {
        let mut temp_longest = 1;
        let mut i = 1;
        let mut j = 1;

        while seen.contains(&(num + i)) {
            temp_longest += 1;
            i += 1;
        }

        while seen.contains(&(num - j)) {
            temp_longest += 1;
            j += 1;
        }

        longest = longest.max(temp_longest);
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neetcode_example_1() {
        let nums = vec![2, 20, 4, 10, 3, 4, 5];
        assert_eq!(longest_consecutive(&nums), 4);
    }

    #[test]
    fn neetcode_example_2() {
        let nums = vec![0, 3, 2, 5, 4, 6, 1, 1];
        assert_eq!(longest_consecutive(&nums), 7);
    }

    #[test]
    fn no_consecutive_sequence() {
        let nums = vec![10, 100, 1000, 10000];
        assert_eq!(longest_consecutive(&nums), 1);
    }

    #[test]
    fn empty_array() {
        let nums: Vec<i64> = vec![];
        assert_eq!(longest_consecutive(&nums), 0);
    }

    #[test]
    fn single_element() {
        let nums = vec![42];
        assert_eq!(longest_consecutive(&nums), 1);
    }

    #[test]
    fn duplicates() {
        let nums = vec![1, 2, 2, 3, 4];
        assert_eq!(longest_consecutive(&nums), 4);
    }

    #[test]
    fn large_range() {
        let nums = vec![-1, 0, 1, 2, 3, 4, 5, 100];
        assert_eq!(longest_consecutive(&nums), 7);
    }

    #[test]
    fn large_gap() {
        let nums = vec![10, 20, 30, 40];
        assert_eq!(longest_consecutive(&nums), 1);
    }
}

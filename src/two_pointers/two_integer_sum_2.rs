/*

    Medium

    Given an array of integers numbers that is sorted in non-decreasing order.
    Return the indices (1-indexed) of two numbers, [index1, index2], such that they add up
    to a given target number target and index1 < index2. Note that index1 and index2 cannot
    be equal, therefore you may not use the same element twice.
    There will always be exactly one valid solution.
    Your solution must use O(1) additional space.

    Example 1:
    Input: numbers = [1,2,3,4], target = 3
    Output: [1,2]
    Explanation:
    The sum of 1 and 2 is 3. Since we are assuming a 1-indexed array, index1 = 1,
    index2 = 2. We return [1, 2].

    Constraints:
    2 <= numbers.length <= 1000
    -1000 <= numbers[i] <= 1000
    -1000 <= target <= 1000

*/

fn two_sum(nums: &[i64], target: i64) -> (usize, usize) {
    let mut first_index = 0;
    let mut second_index = nums.len() - 1;

    while first_index < second_index {
        let current_sum = nums[first_index] + nums[second_index];
        if current_sum > target {
            second_index -= 1;
        } else if current_sum < target {
            first_index += 1;
        } else {
            first_index += 1;
            second_index += 1;
            break;
        }
    }

    (first_index, second_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neetcode_example() {
        let numbers = [1, 2, 3, 4];
        let target = 3;
        assert_eq!(two_sum(&numbers, target), (1, 2));
    }

    #[test]
    fn sum_at_beginning() {
        let numbers = [2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(&numbers, target), (1, 2));
    }

    #[test]
    fn sum_in_the_middle() {
        let numbers = [3, 2, 8, 12, 15];
        let target = 20;
        assert_eq!(two_sum(&numbers, target), (3, 4));
    }

    #[test]
    fn with_negative_nums() {
        let numbers = [-3, 1, 4, 7, 10];
        let target = 7;
        assert_eq!(two_sum(&numbers, target), (1, 5));
    }

    #[test]
    fn sum_involving_negative_num() {
        let numbers = [-10, -3, 4, 8, 12];
        let target = 5;
        assert_eq!(two_sum(&numbers, target), (2, 4));
    }

    #[test]
    fn sum_at_the_end() {
        let numbers = [1, 3, 5, 9, 14];
        let target = 17;
        assert_eq!(two_sum(&numbers, target), (2, 5));
    }
}

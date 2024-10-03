/*

    MEDIUM

    Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
    where nums[i] + nums[j] + nums[k] == 0, and the indices i, j and k are all distinct.
    The output should not contain any duplicate triplets. You may return the output and the triplets in any order.

    Example 1:
    Input: nums = [-1,0,1,2,-1,-4]
    Output: [[-1,-1,2],[-1,0,1]]
    Explanation:
    nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    The distinct triplets are [-1,0,1] and [-1,-1,2].

    Example 2:
    Input: nums = [0,1,1]
    Output: []
    Explanation: The only possible triplet does not sum up to 0.

    Example 3:
    Input: nums = [0,0,0]
    Output: [[0,0,0]]
    Explanation: The only possible triplet sums up to 0.

    Constraints:
    3 <= nums.length <= 1000
    -10^5 <= nums[i] <= 10^5

*/

fn three_sum(nums: &[i64]) -> Vec<(i64, i64, i64)> {
    let mut triplets = Vec::new();

    let mut nums_sorted = nums.to_owned();
    nums_sorted.sort();

    for (i, num) in nums_sorted.iter().enumerate() {
        if nums_sorted[i] > 0 {
            break;
        }

        if i > 0 && nums_sorted[i] == nums_sorted[i - 1] {
            continue;
        }

        let mut second_i = i + 1;
        let mut third_i = nums_sorted.len() - 1;

        while second_i < third_i {
            let current_sum = nums_sorted[i] + nums_sorted[second_i] + nums_sorted[third_i];
            if current_sum > 0 {
                third_i -= 1;
            } else if current_sum < 0 {
                second_i += 1;
            } else {
                triplets.push((nums_sorted[i], nums_sorted[second_i], nums_sorted[third_i]));

                while nums_sorted[third_i] == nums_sorted[third_i - 1] && second_i < third_i {
                    third_i -= 1;
                }
                second_i += 1;
                third_i -= 1;
            }
        }
    }

    triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neetcode_example1() {
        let nums = [-1, 0, 1, 2, -1, -4];
        assert_eq!(three_sum(&nums), vec![(-1, -1, 2), (-1, 0, 1)]);
    }

    #[test]
    fn neetcode_example2() {
        let nums = [0, 1, 1];
        assert_eq!(three_sum(&nums), vec![]);
    }

    #[test]
    fn neetcode_example3() {
        let nums = [0, 0, 0];
        assert_eq!(three_sum(&nums), vec![(0, 0, 0)]);
    }

    #[test]
    fn no_triplets() {
        let nums = [1, 2, -2, -1];
        assert_eq!(three_sum(&nums), vec![]);
    }

    #[test]
    fn multiple_identical_elements() {
        let nums = [-2, -2, 0, 0, 2, 2];
        assert_eq!(three_sum(&nums), vec![(-2, 0, 2)]);
    }

    #[test]
    fn all_negative() {
        let nums = [-3, -2, -1, -4];
        assert_eq!(three_sum(&nums), vec![]);
    }

    #[test]
    fn all_positive() {
        let nums = [1, 2, 3, 4];
        assert_eq!(three_sum(&nums), vec![]);
    }

    #[test]
    fn mixed_elements() {
        let nums = [-1, -1, 2, 2];
        assert_eq!(three_sum(&nums), vec![(-1, -1, 2)]);
    }
}

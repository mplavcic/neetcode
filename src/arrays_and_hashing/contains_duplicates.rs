/*
 * Duplicate Integer:
 * Given an integer array nums, return true if any value appears more than
 * once in the array, otherwise return false.
 */
use std::collections::HashSet;

fn has_duplicate(nums: &[u64]) -> bool {
    let mut container: HashSet<u64> = HashSet::new();
    for num in nums {
        if !container.insert(*num) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = [1, 2, 3, 4];
        assert_eq!(has_duplicate(&nums), false);
    }

    #[test]
    fn test2() {
        let nums = [1, 2, 2, 4];
        assert_eq!(has_duplicate(&nums), true);
    }
}

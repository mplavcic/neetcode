/*

    MEDIUM

    You are given an integer array heights where heights[i] represents
    the height of the i-th bar. You may choose any two bars to form a container.
    Return the maximum amount of water a container can store.

    Example 1:
    Input: height = [1,7,2,5,4,7,3,6]
    Output: 36

    Example 2:
    Input: height = [2,2,2]
    Output: 4

    Constraints:
    2 <= height.length <= 1000
    0 <= height[i] <= 1000

*/

fn max_area(heights: &[usize]) -> usize {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = heights.len() - 1;

    while left < right {
        let temp_area = (right - left) * heights[left].min(heights[right]);
        max_area = max_area.max(temp_area);

        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neetcode_example_1() {
        let heights = vec![1, 7, 2, 5, 4, 7, 3, 6];
        assert_eq!(max_area(&heights), 36);
    }

    #[test]
    fn neetcode_example_2() {
        let heights = vec![2, 2, 2];
        assert_eq!(max_area(&heights), 4);
    }

    #[test]
    fn single_bar_pair() {
        let heights = vec![1, 1];
        assert_eq!(max_area(&heights), 1);
    }

    #[test]
    fn large_heights() {
        let heights = vec![1000, 1000, 1000];
        assert_eq!(max_area(&heights), 2000);
    }

    #[test]
    fn increasing_heights() {
        let heights = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(max_area(&heights), 16);
    }

    #[test]
    fn decreasing_heights() {
        let heights = vec![8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(max_area(&heights), 16);
    }

    #[test]
    fn mixed_heights() {
        let heights = vec![2, 9, 4, 7, 1, 5, 8];
        assert_eq!(max_area(&heights), 40);
    }

    #[test]
    fn equal_heights() {
        let heights = vec![5, 5, 5, 5, 5];
        assert_eq!(max_area(&heights), 20);
    }
}

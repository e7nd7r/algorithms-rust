use std::cmp::{min, max};

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right  {
            max_area =  max(max_area, min(height[left], height[right]) * (right - left) as i32);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::container_most_water::Solution;

    #[test]
    fn test_max_area_irregular() {
        let heights = vec![1,8,6,2,5,4,8,3,7];

        let area = Solution::max_area(heights);

        assert_eq!(area, 49);
    }

    #[test]
    fn test_max_area_repeated() {
        let heights = vec![1,1];
        let area = Solution::max_area(heights);

        assert_eq!(area, 1)
    }

    #[test]
    fn test_max_area_ascending() {
        let heights = vec![1, 2, 3, 4, 5];
        let area = Solution::max_area(heights);

        assert_eq!(area, 6)
    }

    #[test]
    fn test_max_area_descending() {
        let heights = vec![5, 4, 3, 2, 1];
        let area = Solution::max_area(heights);

        assert_eq!(area, 6)
    }


    #[test]
    fn test_max_area_concave() {
        let heights = vec![5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5];
        let area = Solution::max_area(heights);

        assert_eq!(area, 50)
    }

    #[test]
    fn test_max_area_adyancent_maximum() {
        let heights = vec![2, 3, 4, 5, 18, 17, 6];
        let area = Solution::max_area(heights);

        assert_eq!(area, 17)
    }

    #[test]
    fn test_max_area_adyancent_middle_max() {
        let heights = vec![1,2,1];
        let area = Solution::max_area(heights);

        assert_eq!(area, 2)
    }
}

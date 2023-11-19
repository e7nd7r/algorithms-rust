use std::cmp::{min, max};

struct Solution;

/*
    
    [1,8,6,2,5,4,8,3,7]
    [1,1,8,8,8,8,8,8,8]
    [1,]
*/
impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left_max:Vec<usize> = vec![0; height.len()];
        let mut right_max:Vec<usize> = vec![0; height.len()];

        left_max[0] = 0;
        right_max[height.len() - 1] = height.len() - 1;

        for i in 1 .. height.len()  {
            let left_max_index = left_max[i - 1];
            left_max[i] = if height[i] > height[left_max_index as usize] {
                i
            } else 
            {
                left_max_index
            };

            let j = height.len() - i - 1;
            let right_max_index = right_max[j + 1];
            
            right_max[j] = if height[j] > height[right_max_index as usize] {
                j
            } else {
                right_max_index
            }
        }


        for i in 0 .. height.len() {
            let left = left_max[i];
            let right = right_max[i];

            let distance = (right - left) as i32;
            let height = min(height[left], height[right]);

            max_area = max(max_area, distance * height);
        }

        println!("{:?}", left_max);
        println!("{:?}", right_max);

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

        assert_eq!(area, 1)
    }
}

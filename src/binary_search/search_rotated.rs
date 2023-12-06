
struct Solution {
    
}

impl Solution {

    pub(crate) fn find_min(nums: &Vec<i32>) -> usize {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        right
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = Self::find_min(&nums);

        let mut left = if target >= nums[pivot] && target <= *nums.last().unwrap() { 
            pivot as i32
        } else {
            0i32
        };

        let mut right = if target >= nums[pivot] && target <= *nums.last().unwrap() { 
            (nums.len() - 1) as i32 
        } else {
            pivot as i32
        };

        while left <= right {
            let mid = ((left + right) / 2) as i32;

            if nums[mid as usize] == target {
                return mid as i32;
            }

            if target > nums[mid as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::search_rotated::Solution;

    #[test]
    fn test_search01() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
    }

    #[test]
    fn test_search02() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
    }

    #[test]
    fn test_search03() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn test_search04() {
        assert_eq!(Solution::search(vec![0,1,2,4,5,6,7], 0), 0);
    }
}
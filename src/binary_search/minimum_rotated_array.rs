#[allow(dead_code)]
struct Solution {    
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_min(nums: Vec<i32>) -> i32 {
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

        nums[right]
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::minimum_rotated_array::Solution;

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![3,4,5,1,2]), 1);
    }

    #[test]
    fn test_find_min02() {
        assert_eq!(Solution::find_min(vec![4,5,6,7,0,1,2]), 0);
    }

    #[test]
    fn test_find_min03() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_find_min4() {
        assert_eq!(Solution::find_min(vec![1,2,4,5,6,7, 0]), 0);
    }
}
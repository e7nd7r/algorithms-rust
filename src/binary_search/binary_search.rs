struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0i32;
        let mut end = nums.len() as i32 - 1;
        
        while start <= end {
            let index = (start + end) / 2;

            if nums[index as usize] == target {
                return index as i32
            }

            if target < nums[index as usize] {
                end = index - 1;
            } else {
                start = index + 1
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::binary_search::Solution;

    #[test]
    fn test_binary() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
    }

    #[test]
    fn test_binary_no_found_left() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], -2), -1);
    }

    #[test]
    fn test_binary_no_found_right() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 13), -1);
    }

    #[test]
    fn test_binary_one_element() {
        assert_eq!(Solution::search(vec![5], 5), 0);
    }
}
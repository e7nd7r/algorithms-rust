use std::{collections::HashSet, cmp::max};

#[allow(dead_code)] 
struct Solution {}

impl Solution {
    #[allow(dead_code)] 
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<_> = nums.iter().collect();
        let mut max_len = 0;

        for num in &nums {
            if set.contains(&(num - 1)){
                continue;
            }

            let mut current_num = *num;
            let mut current_len = 1;

            while set.remove(&(current_num + 1)) {
                current_len += 1;
                current_num += 1;
            }

            max_len = max(max_len, current_len);

        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use crate::array_hashing::longest_consecutive::Solution;

    // {100, 4, 200, 1, 3, 2}

    #[test]
    fn longest_consecutive_works() {
        let res = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert!(res == 4);
    }

    #[test]
    fn longest_consecutive_works_2() {
        // 0, 3, 7, 2, 5, 8, 4, 6, 1
        let res = Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(res, 9);
    }
}
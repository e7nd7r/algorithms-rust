/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

https://leetcode.com/problems/two-sum/description/
*/
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len() / 2);

        for (i, item) in nums.iter().enumerate() {
            if map.contains_key(item) {
                return vec![map[item], i as i32]
            }
        
            map.insert(target - *item, i as i32);    
        }

        panic!("Invalid state")
    }

    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a hash map to store the difference between target and each number in nums
        let mut hm = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            // Check if such a difference exists in the hash map
            match hm.get(&num) {
                // If it does, return the indices of the current number and the number with the difference
                Some(&j) => return vec![i as i32, j as i32],
                // If it doesn't, add the difference between target and the current number to the hash map
                None => {
                    hm.insert(target - num, i);
                },
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = vec![2,7,11,15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![2, 7]);
    }

    #[test]
    fn test_02() {
        let nums = vec![3,2,4];
        let target = 6;

        assert_eq!(Solution::two_sum(nums, target), vec![2, 4]);
    }
}
/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

 

Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
Example 2:

Input: strs = [""]
Output: [[""]]
Example 3:

Input: strs = ["a"]
Output: [["a"]]
*/

pub struct Solution;

impl Solution {

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::<i32>::with_capacity(nums.len());
        
        let zeros = nums.iter().filter(|n| **n == 0).count();
        
        let mult = nums
            .iter()
            .filter(|n| **n != 0)
            .fold(1, |acc, n| acc * n);
        
        for i in 0..nums.len() {
            answer.push(
                if zeros > 1 || zeros == 1  && nums[i] != 0 {
                    0
                } else if zeros == 1 && nums[i] == 0 {
                    mult
                } else {
                    mult / nums[i]
                }
            )
        }

        answer
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = vec![1,2,3,4];
        let expected = vec![24,12,8,6];
        let result = Solution::product_except_self(nums);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_02() {
        let nums = vec![-1,1,0,-3,3];
        let expected = vec![0,0,9,0,0];
        let result = Solution::product_except_self(nums);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_03() {
        let nums = vec![-1,0,0,-3,3];
        let expected = vec![0,0,0,0,0];
        let result = Solution::product_except_self(nums);

        assert_eq!(expected, result)
    }

}
/*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: falses

Constraints:

1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.

https://leetcode.com/problems/valid-anagram/
*/

use std::collections::{HashMap};

pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        !s.as_bytes()
            .iter()
            .enumerate()
            .fold(&mut [0; 26], |acc, (index, chr)| {
                acc[*chr as usize - 97] += 1;
                acc[t.as_bytes()[index] as usize - 97] -= 1;
                acc
            })
            .iter()
            .any(|c| *c != 0)
    }

    pub fn is_anagram_2(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::<char, i32>::with_capacity(s.len());
        
        Self::count_chars(&mut map, s, 1);
        Self::count_chars(&mut map, t, -1);

        return !map.values().any(|v| *v != 0);
    }

    pub fn is_anagram_3(s: String, t: String) -> bool {
        let a = s.chars().fold([0; 26], |mut acc, x| {acc[(x as u8 - 97) as usize] += 1; acc});
        let b = t.chars().fold([0; 26], |mut acc, x| {acc[(x as u8 - 97) as usize] += 1; acc});
        a.iter().zip(b).fold(true, |acc, (i, j)| acc && (*i == j))
    }

    fn count_chars(map: &mut HashMap<char, i32>, str: String, step: i32)
    {
        for chr in str.chars() {
            let current_value = match map.get(&chr){
                Some(val) => val + step,
                None => 1
            };

            map.insert(chr, current_value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let s = "anagram";
        let t = "nagaram";

        assert!(Solution::is_anagram(s.to_string(), t.to_string()));
    }

    #[test]
    fn test_02() {
        let s = "rat";
        let t = "car";

        assert!(!Solution::is_anagram(s.to_string(), t.to_string()));
    }
}
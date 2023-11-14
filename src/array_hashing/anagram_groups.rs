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

use std::collections::HashMap;

pub struct Solution;

impl Solution {

    pub fn group_anagrams_0(_strs: Vec<String>) -> Vec<Vec<String>> {
        let _bin_repr = HashMap::from([
            ('a', 0x00000001),
            ('b', 0x00000002),
            ('c', 0x00000004),
            ('d', 0x00000008),
            ('e', 0x00000010),
            ('f', 0x00000020),
            ('g', 0x00000040),
            ('h', 0x00000080),
            ('i', 0x00000100),
            ('j', 0x00000200),
            ('k', 0x00000400),
            ('l', 0x00000800),
            ('m', 0x00001000),
            ('n', 0x00002000),
            ('o', 0x00004000),
            ('p', 0x00008000),
            ('q', 0x00010000),
            ('r', 0x00020000),
            ('s', 0x00040000),
            ('t', 0x00080000),
            ('u', 0x00100000),
            ('v', 0x00200000),
            ('w', 0x00400000),
            ('x', 0x00800000),
            ('y', 0x01000000),
            ('z', 0x02000000),
        ]);

        todo!()
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let encoded = strs
            .iter()
            .map(|word| {
                let word = word.to_owned();
                let mut bytes = word.as_bytes().to_owned();

                bytes.sort_unstable();

                String::from_utf8(bytes.to_vec()).unwrap()
            });

        let mut map = HashMap::<String, Vec<String>>::new();
        
        for (i, enc_str) in encoded.enumerate() {
            let mut group = map.get(&enc_str).or(Some(&Vec::new())).unwrap().to_owned();

            group.push(strs[i].clone());

            map.insert(enc_str.clone(), group);
        }
        
        map
            .values()
            .map(|x| x.to_owned())
            .collect()
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()];

        let expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]];

        let result = Solution::group_anagrams(strs);

        let correct = expected
            .iter()
            .all(|ex| 
                result.iter().any(|res| 
                    ex.iter().all(|ex_str| res.contains(ex_str))));
            
        assert!(correct);
    }
}
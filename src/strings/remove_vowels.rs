#[allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn remove_vowels(s: String) -> String {
        s.chars()
            .filter(|c| !['a', 'e', 'i', 'o', 'u'].contains(c))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::strings::remove_vowels::Solution;

    #[test]
    fn remove_vowels_mixed() {
       assert_eq!(Solution::remove_vowels("elviajerosolitario".to_owned()), "lvjrsltr") 
    }

    #[test]
    fn remove_vowels_empty() {
       assert_eq!(Solution::remove_vowels("".to_owned()), "") 
    }

    #[test]
    fn remove_vowels_just_vowels() {
       assert_eq!(Solution::remove_vowels("aeiouaeiou".to_owned()), "") 
    }

    #[test]
    fn remove_vowels_just_consonants() {
       assert_eq!(Solution::remove_vowels("bcdfg".to_owned()), "bcdfg") 
    }
}
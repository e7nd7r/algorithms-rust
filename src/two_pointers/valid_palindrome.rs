struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome_2(s: String) -> bool {
        let iter = s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        iter.clone().eq(iter.rev())
    }

    #[allow(dead_code)]
    pub fn is_palindrome(s: String) -> bool {
        let mut s_chars = s.chars().filter(|&ch| ch.is_alphanumeric()).map(|ch| ch.to_ascii_lowercase());

        while let Some(front) = s_chars.next() {
            if let Some(back) = s_chars.next_back() {
                if front != back {
                    return false;
                }
            }
        }
    
        true
    }

    #[allow(dead_code)]
    pub fn is_palindrome_mine(s: String) -> bool {
        let chars:Vec<_> = s
            .to_ascii_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        
        if chars.len() == 0 {
            return true;
        }

        let mut i = 0 as usize;
        let mut j = chars.len() - 1;

        while i < j {

            if chars[i] != chars[j] {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::valid_palindrome::Solution;

    #[test]
    fn test_is_palindrome() {
        let s = "A man, a plan, a canal: Panama".to_owned();

        assert_eq!(Solution::is_palindrome(s), true);
    }

    #[test]
    fn test_is_palindrome_2() {
        let s = "race a car".to_owned();

        assert_eq!(Solution::is_palindrome(s), false);
    }

    #[test]
    fn test_is_palindrome_3() {
        let s = " ".to_owned();

        assert_eq!(Solution::is_palindrome(s), true);
    }

    #[test]
    fn test_is_palindrome_4() {
        let s = "a".to_owned();

        assert_eq!(Solution::is_palindrome(s), true);
    }

    #[test]
    fn test_is_palindrome_5() {
        let s = "0P".to_owned();

        assert_eq!(Solution::is_palindrome(s), false);
    }
    
}
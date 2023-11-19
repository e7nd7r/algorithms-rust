pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut stack:Vec<char> = Vec::with_capacity(s.len() / 2);
        let mut iter = s.chars();

        while let Some(c) = iter.next() {
            if c == '[' || c == '{' || c == '(' {
                stack.push(c);
                continue;
            }

            if stack.is_empty() && !['[', '(', '{'].contains(&c) {
                return false;
            }

            if !stack.is_empty() {
                if c == ']' && *stack.last().unwrap() != '[' {
                    return false;
                }

                if c == ')' && *stack.last().unwrap() != '(' {
                    return false;
                }

                if c == '}' && *stack.last().unwrap() != '{' {
                    return false;
                }

                stack.pop();
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::valid_parenthesis::Solution;

    #[test]
    pub fn test_parenthesis() {
        assert_eq!(Solution::is_valid("()".to_owned()), true);
    }

    #[test]
    pub fn test_square_brackets() {
        assert_eq!(Solution::is_valid("[]".to_owned()), true);
    }

    #[test]
    pub fn test_brackets() {
        assert_eq!(Solution::is_valid("{}".to_owned()), true);
    }

    #[test]
    pub fn test_wrong_opening_closing() {
        assert_eq!(Solution::is_valid("[}".to_owned()), false);
        assert_eq!(Solution::is_valid("{]".to_owned()), false);
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
        assert_eq!(Solution::is_valid("[)".to_owned()), false);
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
        assert_eq!(Solution::is_valid("{)".to_owned()), false);
        assert_eq!(Solution::is_valid("(}".to_owned()), false);

        assert_eq!(Solution::is_valid("((()))}".to_owned()), false);
    }

    #[test]
    pub fn test_opened_closed_several_different_depths() {
        assert_eq!(Solution::is_valid("((())){}[[[[]]]]({[]})".to_owned()), true);
    }
}

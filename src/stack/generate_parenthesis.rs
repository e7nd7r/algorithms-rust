/*
    ((()))","(()())","(())()","()(())","()()()
 */

use std::collections::HashSet;

struct Solution {}

impl Solution {

    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {

        if n == 1 {
            return vec!["()".to_owned()];
        }
    
        let current_result = &Solution::generate_parenthesis(n - 1);

        let mut result = HashSet::new();
      
        for n_result in current_result {
            let inner = format!("({})", n_result);
            let left = format!("(){}", n_result);
            let right = format!("{}()", n_result);
            
            result.insert(inner);
            result.insert(left);
            result.insert(right);
        }

        result.iter().cloned().collect()
    }
}

/*

*/
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Solution;

    /*
        f(0) = ''
        f(1) = ()
        f(2) = (f(1)) | ()f(1) | f(1)()
        f(3) = '(' ')' '(' f(2) ')' | '(' f(2) ')' '(' ')'  = '(' '(' f(1) ')' ')' = '(' '(' '( f(0) )' ')' ')'
     */
    #[test]
    fn test_parenthesis_generator3() {
        let group:HashSet<String> = HashSet::from_iter(vec!["((()))","(()())","(())()","()()()","()(())"]
            .iter().map(|x|x.to_string()));

        let h: HashSet<String> = HashSet::from_iter(Solution::generate_parenthesis(3).iter().cloned());
        

        assert_eq!(h, group);
    }

    #[test]
    fn test_parenthesis_generator1() {
        let group:HashSet<String> = HashSet::from_iter(vec!["()"]
            .iter().map(|x|x.to_string()));

        let h: HashSet<String> = HashSet::from_iter(Solution::generate_parenthesis(1).iter().cloned());
        

        assert_eq!(h, group);
    }

    #[test]
    fn test_parenthesis_generator4() {
        let group:HashSet<String> = HashSet::from_iter(vec!["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"]
            .iter().map(|x|x.to_string()));

        let h: HashSet<String> = HashSet::from_iter(Solution::generate_parenthesis(4).iter().cloned());
        

        assert_eq!(h, group);
    }
}

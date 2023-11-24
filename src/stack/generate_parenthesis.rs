/*
    ((()))","(()())","(())()","()(())","()()()
 */
struct Solution {}

impl Solution {

    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn generate(ans: &mut Vec<String>, current_str: &mut String, n: i32) {                
            let left_count: usize = current_str
                .chars()
                .fold(0, |s, c| s + (c == '(') as usize);
            let right_count = current_str.len() - left_count;

            if right_count == n as usize {
                ans.push(current_str.clone());
                return ();
            } 
            
            if left_count < n as usize {
                current_str.push('(');
                generate(ans, current_str, n);
                current_str.pop();
            }

            if left_count > right_count as usize {
                current_str.push(')');
                generate(ans, current_str, n);
                current_str.pop();
            }
        }

        let ans = &mut Vec::new();

        generate(ans, &mut "(".to_string(), n);

        ans.to_vec()
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

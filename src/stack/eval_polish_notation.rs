use std::result;

struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut s:Vec<i32> = Vec::with_capacity(tokens.len() / 2);
        let mut iter = tokens.iter().cloned();

        while let Some(token) = iter.next() {
            let val_to_push = match &*token {
                "*" => {          
                    s.pop().unwrap() * s.pop().unwrap()
                }
                "/" => {
                    let right = s.pop().unwrap();
                    let left = s.pop().unwrap();
                    left / right
                }
                "+" => {           
                    let right = s.pop().unwrap();
                    let left = s.pop().unwrap();
                    left + right
                }
                "-" => {           
                    let right = s.pop().unwrap();
                    let left = s.pop().unwrap();
                    left - right
                }
                token => {
                    token.parse::<i32>().unwrap()
                },
            };

            s.push(val_to_push);
        }

        s.pop().unwrap()
        
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_polish_notation() {
        let tokens = "2,1,+,3,*"
            .to_owned()
            .split(",")
            .map(|x| x.to_owned()).collect();

        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn test_polish_notation2() {
        let tokens = "4,13,5,/,+"
            .to_owned()
            .split(",")
            .map(|x| x.to_owned()).collect();

        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn test_polish_notation3() {
        let tokens = "10,6,9,3,+,-11,*,/,*,17,+,5,+"
            .to_owned()
            .split(",")
            .map(|x| x.to_owned()).collect();

        assert_eq!(Solution::eval_rpn(tokens), 22);
    }

    #[test]
    fn test_polish_notation4() {
        let tokens = "4,3,-"
            .to_owned()
            .split(",")
            .map(|x| x.to_owned()).collect();

        assert_eq!(Solution::eval_rpn(tokens), 1);
    }

}
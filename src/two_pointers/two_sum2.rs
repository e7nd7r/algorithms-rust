
#[allow(dead_code)]
struct Solution {

}

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        let mut outer_iter = numbers.iter().enumerate();
        let mut inner_iter = numbers.iter().enumerate();
        let mut back_value = inner_iter.next_back().unwrap();

        while let Some((i, n)) = outer_iter.next() {

            while n + back_value.1 >= target {
                if n + back_value.1  == target {
                    result.append(&mut vec![i as i32 + 1, back_value.0 as i32 + 1]);

                    return result
                } else if n + back_value.1 > target {
                    back_value = inner_iter.next_back().unwrap();
                } else {
                    break;                    
                }
            } 
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::two_sum2::Solution;

    #[test]
    fn test_two_sum() {
        let nums = vec![2,7,11,15];
        let target = 9;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn test_two_sum2() {
        let nums = vec![2,3,4];
        let target = 6;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![1,3]);
    }

    #[test]
    fn test_two_sum3() {
        let nums = vec![-1,0];
        let target = -1;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn test_two_sum4() {
        let nums = vec![5,25,75];
        let target = 100;

        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![2, 3]);
    }
}
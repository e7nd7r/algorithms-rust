struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack : Vec<usize> = Vec::with_capacity(temperatures.len());
        let mut res : Vec<i32> = vec![0; temperatures.len()];

        for (index,temp) in temperatures.iter().enumerate() {
                while let Some(last_temp_index) = stack.last() {
                    if *temp > temperatures[*last_temp_index] {
                        res[*last_temp_index] = (index - *last_temp_index) as i32;
                        stack.pop();
                    } else {
                        break;
                    }
                }

                stack.push(index);
        }

        res
    }

}

#[cfg(test)]
mod tests {
    use crate::stack::daily_temperatures::Solution;

    #[test]
    fn test_dayly_temperatures_01() {
        assert_eq!(Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]), vec![1,1,4,2,1,1,0,0]);
    }

    #[test]
    fn test_dayly_temperatures_02() {
        assert_eq!(Solution::daily_temperatures(vec![30,40,50,60]), vec![1,1,1,0]);
    }

    #[test]
    fn test_dayly_temperatures_03() {
        assert_eq!(Solution::daily_temperatures(vec![30,60,90]), vec![1,1,0]);
    }
}
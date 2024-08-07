use std::cmp::max;

struct Solution {
}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        let mut s: Vec<usize> = Vec::with_capacity(heights.len());
        
        heights.push(0);

        let mut max_area = 0;

        for (index, h) in heights.iter().enumerate() {
            while !s.is_empty() && *h <= heights[*s.last().unwrap()]  {
                let last_index = *s.last().unwrap();
                let height = heights[last_index];

                s.pop();

                let left = s.last().map(|x| *x as i32).unwrap_or(-1);
                let a = (index as i32 - left - 1) * height;

                println!("area: {}", a);

                max_area = max(max_area, a);
            }

            s.push(index)
        }

        max_area
    }   
}

#[cfg(test)]
mod tests {
    use crate::stack::largest_rectangle_histogram::Solution;

    #[test]
    fn test_one_climb() {
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,5,6,2,3]), 10);
    }

    #[test]
    fn test_one_2_elem() {
        assert_eq!(Solution::largest_rectangle_area(vec![2,4]), 4);
    }

    #[test]
    fn test_one_3_elem() {
        assert_eq!(Solution::largest_rectangle_area(vec![2,1,2]), 3);
    }
}
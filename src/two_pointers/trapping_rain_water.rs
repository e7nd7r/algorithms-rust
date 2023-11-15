use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        
        let mut stack = Vec::<usize>::with_capacity(height.len());
        let mut sum = 0;

        for i in 0 .. height.len() {
            let mut max_distance = 0;

            while let Some(stack_index) = stack.iter().last() {
                if height[i] < height[*stack_index] {
                    break;
                }
                
                max_distance = max(max_distance, i as i32 - *stack_index as i32 - 1);

                stack.pop();
            }

            sum += max_distance;
            
            stack.push(i);
        }

        sum
    }

}

#[cfg(test)]
mod tests {
    use crate::two_pointers::trapping_rain_water::Solution;

    #[test]
    pub fn test_trap_concave() {
        let height_map = vec![3, 2, 1, 0, 1, 2, 3, 4];

        assert_eq!(9, Solution::trap(height_map));
    }

    #[test]
    pub fn test_trap_convex() {
        let height_map = vec![1, 2, 3, 2, 1 ];

        assert_eq!(0, Solution::trap(height_map));
    }

    #[test]
    pub fn test_trap_concave_start_zero() {
        let height_map = vec![0, 3, 2, 1, 0, 1, 2, 3, 4];

        assert_eq!(9, Solution::trap(height_map));
    }

    #[test]
    pub fn test_trap_concave_end_zero() {
        let height_map = vec![0, 3, 2, 1, 0, 1, 2, 3, 2, 2];

        assert_eq!(9, Solution::trap(height_map));
    }

    #[test]
    pub fn test_trap_flat() {
        let height_map = vec![0, 0, 0, 0, 0];

        assert_eq!(0, Solution::trap(height_map));
    }

    #[test]
    pub fn test_trap_two_small_holes() {
        let height_map = vec![1, 0, 1, 0, 1];

        assert_eq!(2, Solution::trap(height_map));
    }

    #[test]
    pub fn test_trap_two_with_locals_01() {
        let height_map = vec![0,1,0,2,1,0,1,3,2,1,2,1];

        assert_eq!(6, Solution::trap(height_map));
    }

}

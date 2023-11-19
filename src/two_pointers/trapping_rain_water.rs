use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 { 
        let mut stack = Vec::<usize>::with_capacity(height.len());
        let mut sum = 0;

        for i in 0 .. height.len() {
            let mut max_area = 0;

            while let Some(last_index) = stack.iter().last() {
                if height[i] <= height[*last_index] {
                    break;
                }

                let right_height = height[i];

                let current_index = stack.pop().unwrap();

                let left_index = stack
                    .last()
                    .cloned()
                    .or(Some(current_index)).unwrap();

                let distance = i as i32 - current_index as i32;
               
                let current_height = height[i] - min(right_height, height[left_index]);

                max_area = max(max_area, distance * current_height);
            }

            sum += max_area;
            
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

    #[test]
    pub fn test_trap_highter_tran_steep() {
        let height_map = vec![4,2,0,3,2,5];

        assert_eq!(9, Solution::trap(height_map));
    }
}

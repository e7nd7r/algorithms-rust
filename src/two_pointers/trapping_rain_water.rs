use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 { 
        let left:Vec<i32> = height.iter()
            .enumerate()
            .fold(vec![0; height.len()], |mut acc, (index, h)| {
                acc[index] = match index {
                    0 => *h,
                    _=> max(acc[index - 1], *h)
                };
                acc
            });

        let right:Vec<i32> = height.iter()
            .enumerate()
            .rev()
            .fold(vec![0; height.len()], |mut acc, (index, h)| {
                acc[index] = match index {
                    _ if index == height.len() - 1 => *h,
                    _=> max(acc[index + 1], *h)
                };
                acc
            });
    
        
        height.iter().enumerate()
            .fold(0, |water, (index, current_height)| {
                let boder_height = min(left[index], right[index]);
                water + boder_height - current_height
            })
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

use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut solutions: Vec<Vec<i32>> = Vec::with_capacity(n / 3);
        
        nums.sort();

        for i in 0 .. n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            let (mut j, mut k) = (i + 1, n - 1);
            
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];

                if sum < 0 {
                    j += 1;
                } else if sum > 0 {
                    k -= 1;
                } else {
                    solutions.push(vec![nums[i], nums[j], nums[k]]);
                    
                    j += 1;

                    while nums[j] == nums[j - 1] && j < k {
                        j += 1;
                    }
                }
            }
        }

        return solutions;
    }
}

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // Reallocating memory at each step hurts performace too much.
        let mut existing = HashSet::<i32>::with_capacity(nums.len());
        nums.into_iter().any(|num| !existing.insert(num))
    }

    pub fn contains_duplicate_2(mut nums: Vec<i32>) -> bool {
        // This solution is interesting since it actually performs almost as well than the O(n) solution
        // even when this solution is O(nlog(n))
        nums.sort_unstable();
        nums.windows(2).any(|s| s[0] == s[1])
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = vec![1,2,3,1];

        assert!(Solution::contains_duplicate(nums));
    }

    #[test]
    fn test_02() {
        let nums = vec![1,2,3, 9];

        assert!(!Solution::contains_duplicate(nums));
    }
}

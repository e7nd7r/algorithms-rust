pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut lo = 0;
        let mut hi = (nums.len() - 1) as i32;

        while lo <= hi {
            let mid = (hi - lo) / 2 + hi;

            let count = nums.iter().fold(0, |acc, num| acc + (*num <= mid) as i32);

            if count < mid {
                lo = mid + 1;
            } else {
                ans = mid;
                hi = mid - 1;
            }
        }

        ans
    }
}
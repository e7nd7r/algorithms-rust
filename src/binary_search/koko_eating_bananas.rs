#[allow(dead_code)]
struct Solution {
}

impl Solution {
    pub(crate) fn calculate_total_time(piles: &Vec<i32>, k:i32) -> i32 {
        let mut hours: i32 = 0;
        
        for pile in piles {
            hours += *pile / k + (pile % k != 0) as i32;
        }

        hours
    }

    #[allow(dead_code)]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap_or(&0);

        while left < right {
            let mid: i32 = (left + right) / 2;

            let total_time = Self::calculate_total_time(&piles, mid);

            if total_time <= h {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        right as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::koko_eating_bananas::Solution;

    #[test]
    fn test_min_eating_speed01() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn test_min_eating_speed02() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn test_min_eating_speed03() {
        assert_eq!(Solution::min_eating_speed(vec![30,11,23,4,20], 6), 23);
    }

    #[test]
    fn test_min_eating_speed04() {
        assert_eq!(Solution::min_eating_speed(vec![1000000000], 2), 500000000);
    }
}
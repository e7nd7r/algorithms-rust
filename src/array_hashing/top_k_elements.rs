use std::collections::{HashMap, BinaryHeap };

pub struct Solution;


#[derive(Debug, Eq, Ord)]
pub struct Pair {
    key: i32,
    val: i32
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.val == other.val {
            Some(std::cmp::Ordering::Equal)
        } else if self.val > other.val {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counters = HashMap::new();

        for n in nums {
            *counters.entry(n).or_insert(0) += 1;
        }

        let pairs = counters
            .keys()
            .map(|key| Pair { key: *key, val: *counters.get(key).unwrap()});

        let mut heap = BinaryHeap::<Pair>::new();

        for pair in pairs {
            heap.push(pair)
        }

        let mut result = Vec::<i32>::with_capacity(k as usize);

        for _ in 0..k as usize {
            result.push(heap.pop().unwrap().key)
        }

        result
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;

        let expected = vec![1, 2];
        let result = Solution::top_k_frequent(nums, k);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_02() {
        let nums = vec![1];
        let k = 1;

        let expected = vec![1];
        let result = Solution::top_k_frequent(nums, k);

        assert_eq!(expected, result);
    }
}
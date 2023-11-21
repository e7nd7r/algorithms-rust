use std::collections::HashSet;

#[allow(dead_code)]
struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.iter()
            .map(|str| -> HashSet<_> {
                (0..str.len())
                    .map(|l| -> String { str.chars().take(l + 1).collect() })
                    .collect()
            })
            .reduce(|a, b| a.intersection(&b).cloned().collect())
            .or(Some(HashSet::<String>::new()))
            .unwrap()
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .cloned()
            .or(Some("".to_owned()))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::prefix_tree::longest_common_prefix::Solution;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec!["a".to_owned(), "ab".to_owned()];

        assert_eq!("a".to_owned(), Solution::longest_common_prefix(strs))
    }

    #[test]
    fn test_longest_common_prefix2() {
        let strs = vec!["a".to_owned(), "ab".to_owned()];

        assert_eq!("a".to_owned(), Solution::longest_common_prefix(strs))
    }
}


pub fn wildcard_pattern_matching (pattern: &String, input: &String) -> bool {

    let mut p = 0;
    let mut i = 0;

    let pattern = pattern.as_bytes();
    let input = input.as_bytes();

    while p < pattern.len() && i < input.len() {
        let symbol = pattern[p];
        let c = input[i];

        if (symbol == b'*' || symbol == b'?') && (p < 1 || pattern[p - 1] == b'*' || pattern[p - 1] == b'?') {
            panic!("Invalid expression {:?}", &pattern)
        }

        if symbol == c {
            p += 1; i += 1;

        } else if symbol == b'*' {
            while i < input.len() && input[i] == pattern[p - 1] {
                i += 1;
            }

            p += 1;
        } else if symbol == b'?' {
            if input[i] == pattern[p - 1] {
                i += 1;
            }

            p += 1;
        } else {
            return false;
        }
    }

    if i < input.len() {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::wildcard_pattern_matching;

    #[test]
    fn test_01() {
        let pattern = "aab?".to_string();
        let input = "aab".to_string();

        let result = wildcard_pattern_matching(&pattern, &input);

        assert!(result, "{} matches {}", &pattern, &input);
    }
}
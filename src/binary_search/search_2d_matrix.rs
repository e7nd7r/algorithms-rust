#[allow(dead_code)]
struct Solution {
}

impl Solution {
    #[allow(dead_code)]
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left:i32 = 0;
        let mut right:i32 = (matrix.len() * matrix[0].len()) as i32 - 1;

        while left <= right {
            let mid = ((left + right) / 2) as usize;

            let (i, j) = Self::index2d(mid, (matrix.len(), matrix[0].len()));

            if matrix[i][j] == target {
                return true;
            }

            if target < matrix[i][j] {
                right = mid as i32 - 1;
            } else {
                left = mid as i32 + 1;
            }
        }

        return false;
    }

    pub(crate) fn index2d(index: usize, shape: (usize, usize)) -> (usize, usize) {
        let (_, cols) = shape;

        let row: usize = index / cols;
        let col = index % cols;

        (row, col)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::search_2d_matrix::Solution;

    #[test]
    fn search_matrix01() {
        let matrix = vec![
            vec![ 1, 3, 5, 7],
            vec![10,11,16,20],
            vec![23,30,34,60]
        ];

        assert_eq!(Solution::search_matrix(matrix, 3), true);
    }

    #[test]
    fn search_matrix02() {
        let matrix = vec![
            vec![ 1, 3, 5, 7],
            vec![10,11,16,20],
            vec![23,30,34,60]
        ];

        assert_eq!(Solution::search_matrix(matrix, 13), false);
    }

    #[test]
    fn search_matrix03() {
        let matrix = vec![
            vec![1, 3, 5, 7]
        ];

        assert_eq!(Solution::search_matrix(matrix, 3), true);
    }

    #[test]
    fn search_matrix04() {
        let matrix = vec![
            vec![]
        ];

        assert_eq!(Solution::search_matrix(matrix, 3), false);
    }

    #[test]
    fn test_index2d_traverse() {
        let matrix = vec![
            vec![1,3,5,7],
            vec![10,11,16,20],
            vec![23,30,34,60]
        ];

        let flatten_matrix: Vec<i32> = matrix.iter().cloned().flatten().collect();

        for i in 0..matrix.len() * matrix[0].len() {
            let (j, k) = Solution::index2d(i, (matrix.len(), matrix[0].len()));

            assert_eq!(flatten_matrix[i], matrix[j][k])
        }
    }
}
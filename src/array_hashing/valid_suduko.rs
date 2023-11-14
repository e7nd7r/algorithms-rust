use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = board.len();
        let m = board[0].len();

        if n != 9 || m != 9 {
            panic!("invalid board")
        }

        let mut sets = HashMap::<String, HashSet<char>>::new();
        
        for i in 0 .. n {
            for j in 0 .. m {
                let char = board[i][j];

                if char == '.' {
                    continue;
                }

                let i_q = i / 3;
                let j_q = j / 3;

                let col_key = format!("hl{}", j).to_owned();
                let row_key = format!("vl{}", i).to_owned();
                let quadrant_key = format!("q{},{}", i_q, j_q).to_owned();

                if !sets.contains_key(&col_key) {
                    sets.insert(col_key.clone(), HashSet::new());
                }

                if !sets.contains_key(&row_key) {
                    sets.insert(row_key.clone(), HashSet::new());
                }

                if !sets.contains_key(&quadrant_key) {
                    sets.insert(quadrant_key.clone(), HashSet::new());
                }

                {
                    let row_set = sets.get(&row_key).unwrap();
                    let col_set = sets.get(&col_key).unwrap();
                    let quadrand_set = sets.get(&quadrant_key).unwrap();

                    if row_set.contains(&char) || col_set.contains(&char) || quadrand_set.contains(&char) {
                        return false;
                    }
                }

                if char != '.' {
                    sets.get_mut(&row_key).unwrap().insert(char);
                    sets.get_mut(&quadrant_key).unwrap().insert(char);
                    sets.get_mut(&col_key).unwrap().insert(char);
                }
            }
        }

        true

    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_01() {
        let input = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];

        assert!(Solution::is_valid_sudoku(input));
    }

    #[test]
    fn test_02() {
        let input = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];

        assert!(!Solution::is_valid_sudoku(input));
    }

    #[test]
    fn test_03() {
        let input = vec![
            vec!['.','.','4','.','.','.','6','3','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['5','.','.','.','.','.','.','9','.'],
            vec!['.','.','.','5','6','.','.','.','.'],
            vec!['4','.','3','.','.','.','.','.','1'],
            vec!['.','.','.','7','.','.','.','.','.'],
            vec!['.','.','.','5','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.']
        ];

        assert!(!Solution::is_valid_sudoku(input));
    }

    #[test]
    fn test_04() {
        let input = vec![
            vec!['7','.','.','.','4','.','.','.','.'],
            vec!['.','.','.','8','6','5','.','.','.'],
            vec!['.','1','.','2','.','.','.','.','.'],
            vec!['.','.','.','.','.','9','.','.','.'],
            vec!['.','.','.','.','5','.','5','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','2','.','.'],
            vec!['.','.','.','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','.','.','.','.']
        ];

        assert!(!Solution::is_valid_sudoku(input));
    }

}
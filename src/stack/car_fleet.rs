
#[allow(dead_code)]
struct Solution {
}

#[allow(dead_code)]
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut times: Vec<(i32, f32)> = position
            .iter()
            .enumerate()
            .map(|(index, pos)| (*pos, (target - pos) as f32 / speed[index] as f32)).collect();

        times.sort_unstable_by_key(|elem| elem.0);

        let (counter, _) = times.iter()
            .rev()
            .fold((1, times.last().unwrap().1), |(counter, max), elem| {
                if elem.1 > max {
                    (counter + 1, elem.1)
                } else {
                    (counter, max)
                }
            });
        
        counter
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::car_fleet::Solution;

    #[test]
    fn car_fleet_test01() {
        assert_eq!(Solution::car_fleet(12, vec![10,8,0,5,3], vec![2,4,1,1,3]), 3);
    }

    #[test]
    fn car_fleet_test02() {
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    }

    #[test]
    fn car_fleet_test03() {
        assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2);
    }
}

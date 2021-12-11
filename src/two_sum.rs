// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut collects: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let other: i32 = target - num;

        match collects.get(&other) {
            Some(idx) => {
                return vec![*idx, i as i32];
            }
            None => {
                collects.insert(*num, i as i32);
            }
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}

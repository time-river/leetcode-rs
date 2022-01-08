// https://leetcode.com/problems/jump-game-ii/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut reached = 0usize;
        let mut futurest = nums[0] as usize;
        let mut steps = 0i32;

        for (i, &step) in nums.iter().enumerate() {
            if i > reached {
                steps += 1;
                reached = futurest;
            }

            let jumpest = i + step as usize;
            futurest = std::cmp::max(futurest, jumpest);
        }

        return steps;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 1]), 1);
        assert_eq!(Solution::jump(vec![7,0,9,6,9,6,1,7,9,0,1,2,9,0,3]), 2);
    }
}

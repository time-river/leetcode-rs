// https://leetcode.com/problems/jump-game/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
//        let mut dp: Vec<i32> = vec![0; nums.len()];
//
//        dp[0] = nums[0];
//        for i in 1..nums.len() {
//            let val = i as i32;
//            if dp[i-1] >= val {
//                dp[i] = std::cmp::max(val+nums[i], dp[i-1]);
//            } else {
//                dp[i] = 0i32;
//            }
//        }
//
//        let reached = nums.len() as i32 - 1;
//        return dp[nums.len()-1] >= reached;
        let mut reached = nums[0] as usize;

        for i in 1..nums.len() {
            if i <= reached {
                reached = std::cmp::max(i+nums[i] as usize, reached);
            } else {
                break;
            }
        }

        return reached >= nums.len()-1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![1, 2, 3]), true);
    }
}

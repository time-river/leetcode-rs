// https://leetcode.com/problems/4sum/

use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;

        nums.sort();
        return Self::k_sum(&nums, target, 4);
    }

    fn k_sum(nums: &[i32], target: i32, k: i32) -> Vec<Vec<i32>> {
        if nums.len() < k as usize
            || (nums[0] as i64)*(k as i64) > target as i64
            || (nums[nums.len()-1] as i64)*(k as i64) < target as i64 {
            return Vec::<Vec::<i32>>::new();
        } else if k == 2 {
            return Self::two_sum(nums, target);
        } else {
            let mut ans = Vec::<Vec<i32>>::new();
            let length = nums.len() - (k - 1) as usize;

            for i in 0..length {
                if i > 0 && nums[i] == nums[i-1] {
                    continue;
                }

                for val in Self::k_sum(&nums[i+1..], target-nums[i], k-1).iter_mut() {
                    let mut one = vec![nums[i]];
                    one.append(val);
                    ans.push(one);
                }
            }

            return ans;
        }
    }

    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        // `.map(|key, value)|)` likes `for (key, value) in nums.iter().enumerate()`
        // difference: https://www.zhihu.com/question/263645361
        //   use `map()` to generate new vector
        println!(">> {:?} target: {}", nums, target);
        let bucket: HashMap<i32, usize> = nums.iter().enumerate().map(|(key, value)| {
            return (*value, key);
        }).collect();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let length = nums.len() - 1;

        for i in 0..length {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            match bucket.get(&(target-nums[i])) {
                Some(&j) => {
                    println!(">>> {} {}", j, i);
                    if j > i {
                        ans.push(vec![nums[i], nums[j]]);
                    }
                }
                None => { }
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
                    vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]);
        assert_eq!(Solution::four_sum(vec![0], 0), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::four_sum(vec![0, 0, 0, 0], 0), vec![[0, 0, 0, 0]]);
        assert_eq!(Solution::four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11), vec![[-5, -4, -3, 1]]);
        assert_eq!(Solution::four_sum(vec![-1, 0, -5, -2, -2, -4, 0, 1, -2], -9),
                                       [[-5, -4, -1, 1], [-5, -4, 0, 0], [-5, -2, -2, 0],[-4, -2, -2, -1]]);
        assert_eq!(Solution::four_sum(vec![0, 0, 0, 1000000000, 1000000000, 1000000000, 1000000000], 1000000000),
                                            [[0, 0, 0, 1000000000]]);

    }
}

// https://leetcode.com/problems/next-permutation/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i: i32 = nums.len() as i32 - 2;

        // find the former which is small than laster
        while i >= 0 && nums[i as usize +1] <= nums[i as usize] {
            i -= 1;
        }

        // find the minimal number which is large than `nums[i]` in `nums[i..]`
        if i >= 0 {
            let mut j: i32 = nums.len() as i32 - 1;

            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            nums.swap(j as usize, i as usize);
        }

        let idx = (i + 1) as usize;
        nums[idx..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }
}

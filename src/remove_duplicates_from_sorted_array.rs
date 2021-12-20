// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;

        if nums.len() < 2 {
            return nums.len() as i32;
        }

        for i in 1..nums.len() {
            if nums[i] != nums[idx] {
                idx += 1;
                nums[idx] = nums[i]; 
            }
        }

        return idx as i32 + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(Solution::remove_duplicates(&mut vec![]), 0);
    }
}

// https://leetcode.com/problems/search-in-rotated-sorted-array/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left <= right {
            let mid: usize = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[0] <= nums[mid] && nums[mid] < target {
                left = mid + 1;
            } else if nums[0] <= nums[mid] && nums[0] > target {
                left = mid + 1;
            } else if nums[0] <= nums[mid] && nums[0] <= target {
                right = mid - 1;
            } else if nums[0] > nums[mid] && nums[0] <= target {
                right = mid - 1;
            } else if nums[0] > nums[mid] && nums[0] > target && target > nums[mid] {
                left = mid + 1;
            } else if nums[0] > nums[mid] && nums[0] > target && target < nums[mid] {
                right = mid - 1;
            }
        }

        return -1i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1, 3, 5], 1), 0);
    }
}

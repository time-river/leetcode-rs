// https://leetcode.com/problems/search-insert-position/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len() - 1;

        while lo <= hi {
            let mid: usize = (lo + hi) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                lo = mid + 1;
            } else {
                let tmp = mid as i32 - 1;
                if tmp < 0 {
                    break;
                } else {
                    hi = mid - 1;
                }
            }
        }

        return lo as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}

// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

#[allow(dead_code)]
struct Solution;

impl Solution {
    fn bisect(nums: &Vec<i32>, target: i32, left: bool) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len() - 1;

        while lo <= hi {
            let mid: usize =  (lo + hi) / 2;

            if nums[mid] > target || (left && nums[mid] == target) {
                let tmp: i32 = mid as i32 - 1;

                if tmp < 0 {
                    break;
                } else {
                    hi = mid - 1;
                }
            } else {
                lo = mid + 1;
            }
        }

        return lo as i32;
    }

    #[allow(dead_code)]
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![-1, -1];

        if nums.len() == 0 {
            return ans;
        }

        let left = Self::bisect(&nums, target, true);
        if left < nums.len() as i32 && nums[left as usize] == target {
            ans[0] = left;
            ans[1] = Self::bisect(&nums, target, false) - 1;
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![2, 2], 3), vec![-1, -1]);
    }
}

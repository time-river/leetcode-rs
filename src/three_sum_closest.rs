// https://leetcode.com/problems/3sum-closest/

#[allow(dead_code)]
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    // the same as `let mut ans: i32 = 1000;`
    let mut ans = 1000i32;
    let mut nums = nums;

    nums.sort();
    // https://stackoverflow.com/questions/34717001/whats-the-difference-between-ref-and-when-assigning-a-variable-from-a-referen
    for (i, &_) in nums.iter().enumerate() {
        if i >= nums.len()-2 {
            break;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;

        while j < k {
            let mix = nums[i] + nums[j] + nums[k];
            if (target-mix).abs() < (target-ans).abs() {
                ans = mix;
            }

            if mix > target {
                k -= 1;
            } else if mix == target {
                return ans;
            } else {
                j += 1;
            }
        }
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       // assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
       // assert_eq!(three_sum_closest(vec![1, 1, 1, 1], 0), 3);
       assert_eq!(three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    }
}

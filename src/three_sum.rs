// https://leetcode.com/problems/3sum/

use std::collections::HashMap;

#[allow(dead_code)]
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums;
    let mut bucket: HashMap<i32, usize> = HashMap::new();
    let length = nums.len();

    // if `let mut nums = &mut nums;` is added, it also works.

    nums.sort();

    for i in 0..length {
        bucket.insert(nums[i], i);
    }

    for i in 0..length {
        if nums[i] > 0 {
            break;
        } else if i > 0 && nums[i] == nums[i-1] {
            continue;
        }

        for j in i+1..length {
            if nums[i]+nums[j] > 0 {
                break;
            } else if j > i+1 && nums[j] == nums[j-1] {
                continue;
            }
            
            let target = -(nums[i] + nums[j]);
            match bucket.get(&target) {
                Some(k) => {
                    if *k > j {
                        ans.push(vec![nums[i], nums[j], nums[*k]]);
                    }
                }
                None => { }
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
        assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]),
                                vec![[-1, -1, 2],[-1, 0, 1]]);

        assert_eq!(three_sum(Vec::new()), Vec::<Vec<i32>>::new());

        assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}

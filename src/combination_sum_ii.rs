// https://leetcode.com/problems/combination-sum-ii/

#[allow(dead_code)]
struct Solution;

impl Solution {
    fn dfs(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        for (i, &num) in candidates.iter().enumerate() {
            if i > 0 && candidates[i] == candidates[i-1] {
                continue;
            }

            if target > num {
                let rem = target - num;
                let pathes = Self::dfs(&candidates[i+1..], rem);

                for mut path in pathes.into_iter() {
                    path.push(num);
                    ans.push(path);
                }
            } else if target == num {
                ans.push(vec![target]);
            } else {
                break;
            }
        }

        return ans;
    }

    #[allow(dead_code)]
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        return Self::dfs(&candidates, target);
    }
}

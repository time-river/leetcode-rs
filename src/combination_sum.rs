// https://leetcode.com/problems/combination-sum/

#[allow(dead_code)]
struct Solution;

impl Solution {
    fn dfs(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();

        for (i, &num) in candidates.iter().enumerate() {
            if num < target {
                let rem = target - num;
                let pathes = Self::dfs(&candidates[i..], rem);

                for mut path in pathes.into_iter() {
                    path.push(num);
                    ans.push(path);
                }
            } else if num == target {
                ans.push(vec![num]);
            } else {
                break;
            }
        }

        return ans;
    }

    #[allow(dead_code)]
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        return Self::dfs(&candidates, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let candidates = vec![2, 3, 6, 7];
        let ans = vec![vec![3, 2, 2], vec![7]];
        assert_eq!(Solution::combination_sum(candidates, 7), ans);
    }

}

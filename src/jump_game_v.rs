// https://leetcode.com/problems/jump-game-v/

#[allow(dead_code)]
struct Solution;

impl Solution {
    fn dfs(start: usize, arr: &[i32], d: usize, reached: &mut [i32]) -> i32 {
        let mut idx;
        let mut ans: i32 = 1;

        // use cache
        if reached[start] != 0 {
            return reached[start];
        }

        // find value in left
        idx = start;
        while idx >= 1 && (start-(idx-1)) <= d && arr[idx-1] < arr[start] {
            idx -= 1;
            ans = std::cmp::max(ans, 1+Self::dfs(idx, arr, d, reached));
        }

        // find value in right
        idx = start + 1;
        while idx < arr.len() && (idx - start) <= d && arr[idx] < arr[start] {
            ans = std::cmp::max(ans, 1+Self::dfs(idx, arr, d, reached));
            idx += 1;
        }

        reached[start] = ans;
        return ans;
    }

    #[allow(dead_code)]
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut reached: Vec<i32> = vec![0; arr.len()];
        let mut ans: i32 = 0;

        for (i, _) in arr.iter().enumerate() {
            ans = std::cmp::max(ans, Self::dfs(i, &arr, d as usize, &mut reached));
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
        assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    }
}

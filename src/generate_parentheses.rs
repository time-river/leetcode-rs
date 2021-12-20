// https://leetcode.com/problems/generate-parentheses/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        for i in 0..n {
            let mut lefts = Self::generate_parenthesis(i);

            if lefts.len() == 0 {
                lefts = vec![String::from("")];
            }

            for left in lefts.iter() {
                let mut rights = Self::generate_parenthesis(n-1-i);

                if rights.len() == 0 {
                    rights = vec![String::from("")];
                }

                for right in rights.iter() {
                    let mut s = String::from("(");

                    s.push_str(left);
                    s.push(')');
                    s.push_str(right);
                    ans.push(s);
                }
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}

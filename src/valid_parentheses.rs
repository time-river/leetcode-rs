// https://leetcode.com/problems/valid-parentheses/

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        for &alpha in chars.iter() {
            match alpha {
                '(' | '{' | '[' => {
                    stack.push(alpha);
                }
                ')' if !stack.is_empty() && stack.pop().unwrap() == '(' => {
                    continue;
                }
                '}' if !stack.is_empty() && stack.pop().unwrap() == '{' => {
                    continue;
                }
                ']' if !stack.is_empty() && stack.pop().unwrap() == '[' => {
                    continue
                }
                _ => {
                    return false;
                }
            }
        }

        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }
}

// https://leetcode.com/problems/implement-strstr/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            None => {
                return -1;
            }
            Some(idx) => {
                return idx as i32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::str_str(String::from("hello"), String::from("ll")), 2);
        assert_eq!(Solution::str_str(String::from("h"), String::from("h")), 0);
    }
}

// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;

#[allow(dead_code)]
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ans = 0;
    let chars: Vec<char> = s.chars().collect();
    let (mut start, mut end): (i32, usize) = (-1, 0);
    let mut bucket = HashMap::new();

    while end < chars.len() {
        start = match bucket.get(&chars[end]) {
            // Some(&value) => }
            //    if value > start { value } else { start }
            //}
            Some(value) => {
                if *value > start { *value } else { start }
            }
            None => {
                start
            }
        };
        bucket.insert(chars[end], end as i32);
        let tmp = end as i32 - start;
        if tmp > ans {
            ans = tmp;
        }
        end += 1;
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("abcabcbb");
        let ans = length_of_longest_substring(s);

        assert_eq!(ans, 3);
    }
}

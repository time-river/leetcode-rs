// https://leetcode.com/problems/longest-palindromic-substring/

#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();
    let mut dp: Vec<Vec<bool>> = vec![vec![false; length]; length];
    let mut ans = &chars[0..1];

    if length < 2 {
        return s;
    }

    // The start..end syntax is a Range:
    //   0..length == std::ops::Range { start : 0, end: length }
    for i in 0..length {
        dp[i][i] = true;
    }

    for i in 0..length {
        for j in 0..i {
            // substring [S_j, S_i]
            if chars[i] == chars[j] && (dp[i-1][j+1] || j+1 >= i-1) {
                dp[i][j] = true;
                if i-j+1 > ans.len() {
                    ans = &chars[j..(i+1)];
                }
            } else {
                dp[i][j] = false;
            }
        }
    }

    return ans.iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("babad");
        let ans = longest_palindrome(s);

        assert_eq!(ans, "bab");
    }
}

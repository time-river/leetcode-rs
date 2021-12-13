// https://leetcode.com/problems/string-to-integer-atoi/

#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    // `s.chars()` is the same as `(&s).chars()` because of the auto-dereferencing rules
    let chars: Vec<char> = (&s).chars().collect();
    let mut start = false;
    let mut ans: i64 = 0;
    let mut flag: i64 = 1;

    // `for alpha in chars.iter()`, then
    //   `ans = ans * 10 + (*alpha as u8 - b'0' as u8) as i64;`
    for &alpha in chars.iter() {
        match alpha {
            ' ' => {
                if start {
                    break;
                }
            }
            '+' => {
                if start {
                    break;
                }
                flag = 1;
                start = true;
            }
            '-' => {
                if start {
                    break;
                }
                flag = -1;
                start = true;
            }
            '0'..='9' => {
                start = true;
                ans = ans * 10 + (alpha as u8 - b'0' as u8) as i64;
                if flag == 1 && ans >= (i32::MAX as i64) {
                    ans = i32::MAX as i64;
                    break;
                } else if flag == -1 && (flag * ans) <= (i32::MIN as i64) {
                    ans = i32::MIN as i64;
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
    return (ans * flag) as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(my_atoi("   -42".to_string()), -42);
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(my_atoi("words and 987".to_string()), 0);
        assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(my_atoi("+-12".to_string()), 0);
    }
}

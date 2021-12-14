// https://leetcode.com/problems/palindrome-number/

#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    let mut rev: i32 = 0;
    let mut num: i32 = x;

    if x < 0 {
        return false;
    }

    while num != 0 {
        let rem = num % 10;
        num /= 10;
        rev = rev * 10 + rem;
    }

    return rev == x;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(-1), false);
        assert_eq!(is_palindrome(121), true);
    }
}

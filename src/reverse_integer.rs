// https://leetcode.com/problems/reverse-integer/

#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let mut ans: i64 = 0; // to prevent overflow
    let mut num = x as i64;

    while num != 0 {
        let pop = num % 10;
        num /= 10;

        ans = ans * 10 + pop;
    }

    //if ans >= i32::MIN as i64 && ans <= i32::MAX as i64 {
    if ans >= (-1 << 31) && ans <= (1 << 31) - 1 {
        return ans as i32;
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(-1563847412), 0);
        assert_eq!(reverse(2147483647), 0);
        assert_eq!(reverse(1463847412), 2147483641);
    }
}

// https://leetcode.com/problems/divide-two-integers/

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let m = (dividend as i64).abs();
        let n = (divisor as i64).abs();
        let mut t = n as i64;
        let mut p = 1i64;
        let mut ans = 0i64;

        if m < n {
            return ans as i32;
        } else {
            while m > (t << 1) {
                t <<= 1;
                p <<= 1;
            }

            ans = p + Self::divide((m-t) as i32, n as i32) as i64;

            if (dividend < 0) ^ (divisor < 0) {
                ans = -ans;
            }

            if ans <= i32::MAX.into() {
                ans as i32
            } else {
                i32::MAX
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }
}

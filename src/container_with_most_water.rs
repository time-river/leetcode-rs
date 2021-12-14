// https://leetcode.com/problems/container-with-most-water/

#[allow(dead_code)]
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    let (mut left, mut right): (usize, usize) = (0, height.len()-1);

    while left < right {
        let tmp = std::cmp::min(height[left], height[right]) * (right - left) as i32;
        ans = std::cmp::max(ans, tmp);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(max_area(vec![4,3,2,1,4]), 16);
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }
}

#[allow(dead_code)]
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut ans = std::i32::MAX;
    let mut sub_len = 0usize;
    let mut sum = 0i32;
    let mut i = 0usize;

    for j in 0..nums.len() {
        sum += nums[j];
        while sum >= target {
            sub_len = j - i + 1;
            if ans > sub_len as i32 {
                ans = sub_len as i32;
            }

            sum -= nums[i];
            i += 1;
        }
    }

    if ans == std::i32::MAX {
        return 0;
    } else {
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {

    }
}
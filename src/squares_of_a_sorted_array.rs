#[allow(dead_code)]
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];
    let mut i = 0usize;
    let mut j = nums.len() - 1;
    let mut idx: i32 = j as i32;

    while idx >= 0 {
        let a = nums[i].pow(2);
        let b = nums[j].pow(2);

        if a < b {
            ans[idx as usize] = b;
            j -= 1;
        } else {
            ans[idx as usize] = a;
            i += 1;
        }

        idx -= 1;
    }

    return ans;
}

#[cfg(test)]
mod test {
    use super::*;

    fn it_works() {

    }
}
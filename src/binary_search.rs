#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }

    let mut i: i32 = 0;
    let mut j: i32 = nums.len() as i32 - 1;

    while i <= j {
        let idx = (i + j) / 2;
        if nums[idx as usize] < target {
            i = idx + 1;
        } else if nums[idx as usize] > target {
            j = idx - 1;
        } else {
            return idx;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(search(vec![5], -5), -1);
    }
}

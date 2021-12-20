// https://leetcode.com/problems/zigzag-conversion/

#[allow(dead_code)]
pub fn convert(s: String, num_rows: i32) -> String {
    let length = s.len();
    let chars: Vec<char> = s.chars().collect();
    let mut ans: Vec<String> = vec![String::new(); length];
    let mut idx: i32 = 0;
    let mut going_down = false;

    if length < 2 {
        return s;
    }

    for ch in chars.iter() {
        ans[idx as usize].push(*ch);
        if idx == 0 || idx == num_rows-1  {
            going_down = !going_down;
        }
        idx += if going_down { 1 } else { -1 };
    }

    return ans.join("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("PAYPALISHIRING");
        let ans = convert(s, 3);

        assert_eq!(ans, "PAHNAPLSIIGYIR");
    }
}

// https://leetcode.com/problems/roman-to-integer/

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    let map: Vec<(&str, i32)> = vec![
        ("M", 1000), ("CM", 900), ("D", 500),
        ("CD", 400), ("C", 100), ("XC", 90),
        ("L", 50), ("XL", 40), ("X", 10),
        ("IX", 9), ("V", 5), ("IV", 4),
        ("I", 1)
    ];
    let mut ans: i32 = 0;
    let mut i: usize = 0;
    let slices: &str = s.as_str();
    let length: usize = s.len();

    while i < length {
        for &(key, value) in map.iter() {
            match key.len() {
                1 => {
                    if key == &slices[i..i+1] {
                        ans += value;
                        i += 1;
                        break;
                    }
                }
                2 => {
                    if (length-i) > 1 && key == &slices[i..i+2] {
                        ans += value;
                        i += 2;
                        println!("ans: {}", ans);
                        break;
                    }
                }
                _ => { }
            }
        }
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(roman_to_int(String::from("I")), 1);
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}

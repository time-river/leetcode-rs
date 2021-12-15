// https://leetcode.com/problems/integer-to-roman/

#[allow(dead_code)]
pub fn int_to_roman(num: i32) -> String {
    let map = vec![
        (1, "I"), (4, "IV"), (5, "V"), (9, "IX"),
        (10, "X"), (40, "XL"), (50, "L"), (90, "XC"),
        (100, "C"), (400, "CD"), (500, "D"), (900, "CM"),
        (1000, "M")
    ];
    let mut ans = String::new();
    let mut roman = num;

    while roman != 0 {
        // `map.iter().rev()` type is `&({integer}, &str)`
        // if `for (key, value) in map.iter().rev()`, then key type is `&{integer}`
        for &(key, value) in map.iter().rev() {
            if roman >= key {
                roman -= key;
                ans.push_str(value);
                break;
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
        assert_eq!(int_to_roman(3), "III");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(20), "XX");
    }
}

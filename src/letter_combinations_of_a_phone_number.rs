// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

use std::collections::HashMap;

#[allow(dead_code)]
pub fn letter_combinations(digits: String) -> Vec<String> {
    // #[stable(feature = "std_collections_from_array", since = "1.56.0")]
    // let bucket: HashMap<char, Vec<char>> = HashMap::from([
    //     ('2', vec!['a', 'b', 'c']),
    //     ('3', vec!['d', 'e', 'f']),
    //     ('4', vec!['g', 'h', 'i']),
    //     ('5', vec!['j', 'k', 'l']),
    //     ('6', vec!['m', 'n', 'o']),
    //     ('7', vec!['p', 'q', 'r', 's']),
    //     ('8', vec!['t', 'u', 'v']),
    //     ('9', vec!['w', 'x', 'y', 'z']),
    // ]);
    let bucket: HashMap<char, Vec<char>> = [
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z'])
        ].iter().cloned().collect();
    let chars: Vec<char> = digits.chars().collect();

    return letter_combinations_internal(&chars, &bucket);
}

fn letter_combinations_internal(digits: &[char], bucket: &HashMap<char, Vec<char>>) -> Vec<String> {
    if digits.len() == 0 {
        return Vec::<String>::new();
    } else if digits.len() == 1 {
        let alphas = &bucket[&digits[0]];
        let mut ans: Vec<String> = Vec::new();

        for alpha in alphas.iter() {
            // #[stable(feature = "from_char_for_string", since = "1.46.0")]
            // ans.push(String::from(*alpha));
            ans.push((*alpha).to_string());
        }

        return ans;
    } else {
        let last = letter_combinations_internal(&digits[1..], bucket);
        let alphas = &bucket[&digits[0]];
        let mut ans: Vec<String> = Vec::new();

        for &alpha in alphas.iter() {
            for one in last.iter() {
                let mut s = alpha.to_string();

                s.push_str(&one[..]);
                ans.push(s);
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    }
}

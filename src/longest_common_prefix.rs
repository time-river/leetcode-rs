// https://leetcode.com/problems/longest-common-prefix/

#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ans = String::new();

    if strs.len() == 0 {
        return ans;
    }

    let mut short: &[u8] = strs[0].as_bytes();

    for s in strs.iter() {
        if s.len() < short.len() {
            short = s.as_bytes();
        }
    }

    for (i, ch) in short.iter().enumerate() {
        for s in strs.iter() {
            if s.as_bytes()[i] != *ch {
                return ans;
            }
        }
        ans.push(*ch as char);
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let strs: Vec<String> = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(longest_common_prefix(strs), "fl");
    }
}

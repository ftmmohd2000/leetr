struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() | (strs[0].len() == 0) {
            return "".to_string();
        }

        let mut index = 0;
        for (i, _str) in strs.iter().enumerate() {
            if _str.len() < strs[index].len() {
                index = i;
            }
        }

        for i in 0..strs[index].len() {
            let chr = strs[index].chars().nth(i).unwrap();
            for _str in strs.iter() {
                if let Some(ch) = _str.chars().nth(i) {
                    if chr != ch {
                        return strs[index][..i].to_owned();
                    }
                } else {
                    return strs[index][..i].to_owned();
                }
            }
        }

        strs[index][..].to_owned()
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::Solution;

    fn verify(answer: String, expected: String) {
        assert_eq!(answer, expected);
    }

    //    #[test]
    fn leet_test_cases() {
        verify(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            String::from("fl"),
        );
        verify(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ]),
            String::from(""),
        );
    }
}

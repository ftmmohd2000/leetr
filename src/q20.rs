struct Solution;
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut ref_map = HashMap::new();
        ref_map.insert('}', '{');
        ref_map.insert(')', '(');
        ref_map.insert(']', '[');
        let mut st = vec![];
        for ch in s.chars() {
            match ch {
                '{' | '(' | '[' => st.push(ch),
                '}' | ')' | ']' => {
                    if (st.len() == 0) || (*ref_map.get(&ch).unwrap() != st.pop().unwrap()) {
                        return false;
                    }
                }
                _ => {
                    panic!("Unexpected Character")
                }
            }
        }

        return st.len() == 0;
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::Solution;

    fn verify(answer: bool, expected: bool) {
        assert_eq!(answer, expected);
    }

    //    #[test]
    fn leet_test_cases() {
        verify(Solution::is_valid("()".to_owned()), true);
        verify(Solution::is_valid("(){}[]".to_owned()), true);
        verify(Solution::is_valid("(]".to_owned()), false);
        verify(Solution::is_valid("]".to_owned()), false);
    }
}

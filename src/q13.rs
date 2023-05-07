struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let vals = s.chars().map(|ch| match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        });
        let mut value = 0i32;
        let mut last = None;

        for val in vals {
            value += val;
            if let Some(lvalue) = last {
                if lvalue < val {
                    value -= 2 * lvalue;
                }
            }
            last = Some(val);
        }

        value
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::Solution;

    fn verify(answer: i32, expected: i32) {
        assert_eq!(answer, expected);
    }

    //    #[test]
    fn leet_test_cases() {
        verify(Solution::roman_to_int(String::from("III")), 3);
        verify(Solution::roman_to_int(String::from("LVIII")), 58);
        verify(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}

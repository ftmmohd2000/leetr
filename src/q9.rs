struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let (mut rev, mut og) = (0, x);
        while og > 0 {
            rev = (rev * 10) + (og % 10);
            og /= 10;
        }

        x == rev
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::Solution;

    fn verify(answer: bool, expected: bool) {
        assert_eq!(answer, expected);
    }

//    #[test]
    fn leet_test_cases(){
        verify(
            Solution::is_palindrome(121),
            true
        );
        verify(
            Solution::is_palindrome(-121),
            false
        );
        verify(
            Solution::is_palindrome(10),
            false
        );
    }
}

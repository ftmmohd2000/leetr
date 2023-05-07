
struct Solution;

use std::collections::hash_map::HashMap;
#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ref_map = HashMap::new();
        for (i, num) in nums.iter().enumerate(){
            match ref_map.get(num) {
                Some(&value) => return vec![i as i32, value],
                None => ref_map.insert(target - *num, i as i32)
            };
        }
        vec![]
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::collections::hash_set::HashSet;

    use super::Solution;

    fn verify(answer: Vec<i32>, mut expected: HashSet<i32>) {
        assert_eq!(answer.len(), 2);
        for el in answer{
            expected.remove(&el);
        }
        assert_eq!(expected.len(), 0);
    }

//    #[test]
    fn leet_test_cases(){
        verify(
            Solution::two_sum(vec![2, 7, 11, 15], 9),
            HashSet::from([1, 0])
        );
        verify(
            Solution::two_sum(vec![3, 2, 4], 6),
            HashSet::from([1, 2])
        );
        verify(
            Solution::two_sum(vec![3, 3], 6),
            HashSet::from([1, 0])
        );
    }
}

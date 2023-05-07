struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();

        let mut i = 0;
        while i < (nums.len() - 1) {
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum < 0 {
                    j += 1;
                } else if sum > 0 {
                    k -= 1;
                } else {
                    result.push(vec![nums[i], nums[j], nums[k]]);
                    j = Self::move_right(&nums, j);
                }
            }

            i = Self::move_right(&nums, i);
        }
        result
    }

    fn move_right(arr: &Vec<i32>, mut index: usize) -> usize {
        loop {
            index += 1;
            if (index >= (arr.len() - 1)) | (arr[index] != arr[index - 1]) {
                break;
            }
        }
        index
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    fn verify(mut answer: Vec<Vec<i32>>, expected: HashSet<Vec<i32>>) {
        let mut seen = HashSet::new();
        for ans in answer.iter_mut() {
            assert!(!seen.contains(ans));
            assert!(expected.contains(ans));
            seen.insert(ans);
        }
    }

    //    #[test]
    fn leet_test_cases() {
        verify(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            HashSet::from([vec![-1, -1, 2], vec![-1, 0, 1]]),
        );
        verify(Solution::three_sum(vec![0, 1, 1]), HashSet::from([]));
        verify(
            Solution::three_sum(vec![0, 0, 0]),
            HashSet::from([vec![0, 0, 0]]),
        );
        verify(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            HashSet::from([vec![-2, 0, 2]]),
        );
    }
}

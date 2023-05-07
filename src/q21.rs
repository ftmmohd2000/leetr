#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr = &mut list1;
        while list2.is_some() {
            if curr.is_none() || list2.as_ref()?.val < curr.as_ref()?.val {
                std::mem::swap(curr, &mut list2);
            }
            curr = &mut curr.as_mut()?.next;
        }
        list1
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    fn verify(answer: bool, expected: bool) {
        assert_eq!(answer, expected);
    }

    #[test]
    fn leet_test_cases() {
        // no test I was feeling lazy
    }
}

// ============================
// PREDEFINED
// ============================
pub struct Solution;
use crate::utils::ListNode;
// ============================

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_deref();
        let mut l2 = l2.as_deref();
        let mut result = ListNode { val: 0, next: None };
        let mut tail = &mut result;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            // deref from Option<Box<ListNode>> to Option<&ListNode>
            let a = l1.as_deref().map_or(0, |n| n.val);
            let b = l2.as_deref().map_or(0, |n| n.val);

            let old_carry = carry;

            let sum = a + b + carry;
            let digit = sum % 10;
            carry = sum / 10;

            println!("{a} + {b} w {old_carry} = {digit} with {carry}");

            let new_tail = ListNode {
                val: digit,
                next: None,
            };

            // step next
            l1 = l1.and_then(|n| n.next.as_deref());
            l2 = l2.and_then(|n| n.next.as_deref());
            tail = tail.next.insert(Box::new(new_tail));
        }

        result.next
    }
}

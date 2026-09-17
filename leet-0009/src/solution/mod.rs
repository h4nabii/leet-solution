// ============================
// PREDEFINED
// ============================
pub struct Solution;
// ============================

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // 负数不可能是回文数
        // 如果最后一位是 0，则不可能是回文数，除非数字本身是 0
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut ori = x;
        let mut rev = 0;
        while ori > rev {
            rev = rev * 10 + ori % 10;
            ori /= 10;
        }

        ori == rev || ori == rev / 10
    }
}

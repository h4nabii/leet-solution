// ============================
// PREDEFINED
// ============================
pub struct Solution;
// ============================

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: HashSet<u8> = HashSet::new();
        let mut ans = 0;
        let mut r = 0;

        // 手动控制左右边界的滑动窗口
        // 不要使用 s.iter().windows(n)，效率太低

        let bytes = s.as_bytes();

        for i in 0..s.len() {
            if i != 0 {
                set.remove(&bytes[i - 1]);
            }

            while r < s.len() && !set.contains(&bytes[r]) {
                set.insert(bytes[r]);
                r += 1;

                ans = ans.max(r - i);
            }
        }

        ans as _
    }
}

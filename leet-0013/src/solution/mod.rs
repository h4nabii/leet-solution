// ============================
// PREDEFINED
// ============================
pub struct Solution;
// ============================

impl Solution {
    pub fn translate_roman_to_int(c: u8) -> i32 {
        match c {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => 0,
        }
    }

    /**
     * 转换罗马数字字符串为整数
     * @param s 罗马数字字符串
     * @return 对应的整数值
     */
    pub fn roman_to_int(s: String) -> i32 {
        // 罗马数字中，只存在特定的减法组合，如 IV 表示 4，IX 表示 9 等
        // 减数只能是 10 的倍数，即 I，X，C
        // 基于上述减法规则，也不会存在组合与组合相减的情况
        // 于是可以从右向左遍历罗马数字字符，遇到比前一个字符小的就减去，否则加上

        let s = s
            .as_bytes()
            .iter()
            .map(|&n| Solution::translate_roman_to_int(n))
            .collect::<Vec<i32>>();

        let mut result = 0;
        let mut prev = 0;

        // 反向判断逻辑更简单
        for &value in s.iter().rev() {
            if value < prev {
                result -= value;
            } else {
                result += value;
            }
            prev = value;
        }

        result
    }
}

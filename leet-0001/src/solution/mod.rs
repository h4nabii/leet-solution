use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let t = target - num;
            let result = map.get(&t);

            match result {
                Some(&index) => return vec![index as i32, i as i32],
                None => (),
            }

            map.insert(num, i);
        }

        panic!("No two sum solution found")
    }
}

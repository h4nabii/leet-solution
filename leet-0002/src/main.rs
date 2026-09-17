mod solution;
mod utils;

use solution::Solution;
use utils::ListNode;

fn main() {
    let result = Solution::add_two_numbers(
        ListNode::from_vec(vec![9, 9, 9, 9]),
        ListNode::from_vec(vec![9, 9, 9, 9]),
    );
    println!("{:?}", result);
}

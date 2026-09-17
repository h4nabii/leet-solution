mod solution;

fn main() {
    let result = solution::Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", result);
}

mod solution;

use solution::Solution;

fn main() {
    const EXAMPLES: [&str; 5] = ["III", "IV", "IX", "LVIII", "MCMXCIV"];

    for example in EXAMPLES.iter() {
        let str = example.to_string();
        println!("{str} is {}", Solution::roman_to_int(str.clone()));
    }
}

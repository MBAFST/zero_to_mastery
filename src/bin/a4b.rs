// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively

fn main() {
// * Use a variable set to any integer
    let number: i32 = 3;
// * Use a match expression to determine which message to display
    let msg: &str = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
// * Use an underscore (_) to match on any value
        _ => "Other"
    };
    println!("{msg}");
}

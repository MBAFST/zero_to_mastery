fn main() {
    let number: i32 = 3;
    let msg: &str = match number {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Other"
    };
    println!("{msg}");
}
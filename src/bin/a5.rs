// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal

fn main() {
// * Use a mutable integer variable
    let mut count = 1;
// * Use a loop statement
    loop {
// * Print the variable within the loop statement
        println!("{count}");

        count += 1;
// * Use break to exit the loop
        if count == 4 {
            break;
        }
    }

    println!("{count}");
}

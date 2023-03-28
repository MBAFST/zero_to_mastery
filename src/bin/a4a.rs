// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable

fn main() {
// * Use a variable set to either true or false
    let my_bool = true;
// * Use a match expression to determine which message to display
    let my_bool = match my_bool {
        true => {
            println!("it's true");
            true
        },
        false => {
            println!("it's false");
            true
        }
    };

    println!("{my_bool}")
}

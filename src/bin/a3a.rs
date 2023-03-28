// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"

fn main() {
// * Use a variable set to either true or false
    let is_connected = false;
// * Use an if..else block to determine which message to display
    if is_connected {
// * Use the println macro to display messages to the terminal
        println!("hello");
    }
    else {
// * Use the println macro to display messages to the terminal
        println!("goodbye");
    }
}

// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal

// * Use an enum with color names as variants
enum Color {
    Black,
    Red,
    Green,
    Blue,
    White
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color: Color) {
// * Use a match expression to determine which color
//   name to print
    match color {
        Color::Black => println!("Black"),
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::White => println!("White"),
    }
}

fn main() {
    print_color(Color::Black);
    print_color(Color::Red);
    print_color(Color::Green);
    print_color(Color::Blue);
    print_color(Color::White);
}

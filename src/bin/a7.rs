enum Color {
    Black,
    Red,
    Green,
    Blue,
    White
}

fn print_color(color: Color) {
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

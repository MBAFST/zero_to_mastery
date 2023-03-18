fn main() {
    let my_bool = true;

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

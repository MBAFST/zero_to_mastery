fn main() {
    let mut count = 1;

    loop {
        println!("{count}");

        count += 1;

        if count == 4 {
            break;
        }
    }

    println!("{count}");
}
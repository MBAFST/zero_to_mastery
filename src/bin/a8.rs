// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// * Use an enum to create different flavors of drinks
enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
// * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Sparkling => println!("Flavor: Sparkling"),
        Flavor::Sweet => println!("Flavor: Sweet"),
        Flavor::Fruity => println!("Flavor: Fruity")
    }

    println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
    let first_drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 6_f64
    };

    print_drink(first_drink);

    let second_drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 10_f64
    };

    print_drink(second_drink);

    let thrid_drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 17_f64
    };

    print_drink(thrid_drink);
}

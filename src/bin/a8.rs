enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64
}

fn print_drink(drink: Drink) {
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
    let second_drink = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 10_f64
    };
    let thrid_drink = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 17_f64
    };

    print_drink(first_drink);
    print_drink(second_drink);
    print_drink(thrid_drink);
}

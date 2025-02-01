#[derive(Debug)]
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("I am now drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 100.22
}

fn main() {
    let mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);

    let values: [&str; 2] = ["Hello", "World"];
    // println!("{}", mocha);
    println!("{:?}", mocha);
    println!("{:#?}", mocha);
}

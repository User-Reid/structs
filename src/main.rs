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

fn main() {
    let mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);

    let frapachino: Coffee = Coffee {
        name: String::from("{Frapachino}"),
        ..mocha
    };
}

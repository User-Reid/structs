struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let name: String = String::from("Mocha");
    let coffee: Coffee = make_coffee(name, 5.99, true);

    fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
        Coffee {
            name: name,
            price: price,
            is_hot: is_hot,
        }
    }
    println!("{}, {}, {}", coffee.name, coffee.price, coffee.is_hot)
}

struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let name: String = String::from("Mocha");
    let coffee: Coffee = make_coffee(name, 5.99, true);
    println!("{}, {}, {}", coffee.name, coffee.price, coffee.is_hot);

    let name: String = String::from("Latte");
    let price: f64 = 3.99;
    let is_hot: bool = false;
    let latte: Coffee = Coffee {
        name,
        price,
        is_hot,
    };

    fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
        Coffee {
            name,
            price,
            is_hot,
        }
    }
}

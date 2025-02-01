fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mut mocha: Coffee = Coffee {
        price: 5.99,
        name: String::from("Mocha"),
        is_hot: true,
    };

    println!(
        "My {} this morning cost {}, and indeed it was {} that it was hot.",
        mocha.name, mocha.price, mocha.is_hot
    );

    let favorite_coffee: String = mocha.name;
    mocha.price = 7.99;

    println!("{}", mocha.price)
}

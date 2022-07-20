enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn if_let_ex(ex: Option<u8>) {
    let config_max = ex;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    //    println!("{:?}", if_let_ex(Some(2u8)));
    if_let_ex(Some(3u8));
}

fn main() {
    println!("Hello, world!");

    another_function(1);

    exp();

    return_func();

    call_plus_one();
}

fn another_function(x: u32) {
    println!("Another function. {}", x);
}

fn exp() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn return_func() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn call_plus_one() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

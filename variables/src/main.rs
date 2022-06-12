fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let mut spaces_mut = "    ";
    // spaces_mut = spaces_mut.len();
    println!("{}", spaces_mut);

    let x = 2.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product: u64 = 4 * 3000000000000000000;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup = (1, 2000, "str");
    let (x, y, z) = tup;
    println!("{}", y);
    println!("{}", tup.2);

    // let arr = [1,2,"shogo",4.0];
    let arr:[i32;3] = [1,2,3];
    let aut_arr = [1;5];
    let first_arr = arr[0];
    
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &mut v[0];

    println!("The first element is: {}", first);
    v.push(6);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut numbers: Vec<i32> = vec![1, 2, 3];

    numbers.push(4);

    let third_elem: &i32 = &numbers[1];
    let third_get = numbers.get(2);

    println!("{third_elem}");

    match third_get {
        Some(x) => println!("{}", x),
        None => println!("There is no third element"),
    }
    println!("{:?}", third_get);

    println!("{:?}", numbers);

    // --------------------UTF8--------------------
    let s = "whatever".to_string();
    let s = String::from("whatever");
    let mut s = String::from("foo");

    s.push_str("bar");
    s.push('!'); // push is for a single character only and use single quotes

    println!("{s}");

    // --------------------Hash Maps--------------------
    let mut scores: HashMap<String, u16> = HashMap::new();

    scores.insert("Yellow".to_string(), 10);
    scores.insert("Blue".to_string(), 20);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("Team {} curerntly has {} points!", key, value)
    }
}

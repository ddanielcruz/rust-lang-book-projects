use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    vectors();
    strings();
    maps();
}

fn vectors() {
    // We can create a vector with a type annotation or by using the vec! macro
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    // Update a value in a vector by using dereference operator
    for i in &mut v {
        *i += 50;
    }
    
    println!("v: {:?}", v);

    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    };

    // Enums are considered one type, so we can have a vector with different data types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row);
}

fn strings() {
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = String::from("initial contents");
    let mut s = "initial contents".to_string();
    println!("\ns: {}", s);

    s.push_str(" - updated");
    s.push('!');
    println!("s: {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let sc = format!("{s1}-{s2}-{s3}");
    println!("sc: {}", sc);
}

fn maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 15); // Overwrite existing value
    scores.insert(String::from("yellow"), 20);
    scores.entry(String::from("green")).or_insert(5); // Insert if key does not exist
    println!("\nscores: {:?}", scores);
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("blue score: {}", score);
}

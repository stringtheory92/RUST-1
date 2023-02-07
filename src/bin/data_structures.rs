fn main() {
    // vectors();
    // enums();
    strings();

}


fn vectors() {
    
    println!("Hia");

    let mut v = Vec::new();
    let mut a = vec![1, 2, 3];
    let s = vec!["Here", "We", "Go", "!"];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let other_third = v[2];
    println!("The other third is {other_third:?}");

    v.push(9);
    println!("now try to print v: {v:?}");


    for i in v {
        println!("element: {i}");
    }

    for i in &mut a {
        *i *= 10;
    }

    println!("{:?}", a);
    // println!("{:?}", v);
    // println!("{:?}", s);
}

fn enums() {
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

fn strings() {
    println!("String bool: {}", "xyz".to_string() == String::from("xyz"));

    let mut x = String::from("Hello");
    let y = " world".to_string();
    x.push_str(&y);
    

    println!("xy: {x:?}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}, s1 is {s1}");

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let combined = hello + &world + "-" + &world + "-";
    
    // println!("hello: {hello}, world: {world}, combined: {combined}");
    println!("world: {world}, combined: {combined}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let combined2 = format!("{tic}-{tac}-{toe}");
    // println!("{combined2}");

    for char in combined2.chars() {
        println!("{char}");
    }
}
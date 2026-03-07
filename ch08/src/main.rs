fn main() {
    let mut v = vec![1, 2, 3];
    v.push(3);

    let third = v[2];

    let second = v.get(1);
    let second_a = second.unwrap();

    v.push(3);

    println!("{:?}", v);
    println!("{}", third);
    // println!("{}", second_a);

    for i in &v {
        println!("{i}");
    }

    // String
    let mut s = String::new();
    let s = "initial contents".to_string();
    let mut s = String::from("initial contents");
    s.push_str("bar");

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    // let s3 = s1 + &s2;
    // println!("{s3}");
    let s3 = format!("{s1} {s2}");
    println!("{s1}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");

    if s.is_char_boundary(4) {
        println!("NO");
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    let m = scores.entry(String::from("Blue")).or_insert(50);

    // println!("{scores:?}");
    println!("{m}");
}

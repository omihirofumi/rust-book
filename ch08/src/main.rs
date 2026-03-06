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
}

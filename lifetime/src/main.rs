fn main() {
    let string1 = String::from("abcd");
    let result;

    let string2 = String::from("abcdef");
    result = longest(string1.as_str(), string2.as_str());

    println!("The longest value is {result}");

    let novel = String::from("Call me Ishmael. Some years go...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// fn longest2(x: &str, y: &str) -> &str {
//     x
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

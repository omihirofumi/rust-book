fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];

    let n: i32 = *n_ref;
}

// これはだめ。sが所有権持ってるので、スコープ抜けるとdropされてしまう。
// fn return_a_string_err() -> &str {
//     let s = String::from("Hello world");
//     &s
// }

fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

fn return_a_string_static() -> &'static str {
    "Hello world"
}

// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // ここの参照によって、*dstに対するwrite権限が削除される

//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }
//

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

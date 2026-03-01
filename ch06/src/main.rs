enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("aaaaaaa"))
    //     } else {
    //         Some(format!("aaaaaaaaaaaaaaaaaaaa"))
    //     }
    // } else {
    //     None
    // }
    //
    // リファクタ1
    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    // if state.existed_in(1900) {
    //     Some(format!("aaaaaaa"))
    // } else {
    //     Some(format!("aaaaaaaaaaaaaaaaaaaa"))
    // }
    // リファクタ２
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("aaaaaaa"))
    } else {
        Some(format!("aaaaaaaaaaaaaaaaaaaa"))
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Hello"),
        _ => (),
    }

    // if let PATTERN = EXPR { ... }
    // letはpatternに束縛する。→if letはパターンに束縛できるかどうか
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
    }
}

pub use crate::front_of_house::hosting;

mod front_of_house {
    pub mod hosting {
        use crate::{eat_at_restaurant, front_of_house};

        pub fn add_to_waitlist() {
            front_of_house::eat_at_restaurant();
        }

        fn seat_at_table() {}
    }

    mod serving {
        use crate::front_of_house;

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

mod customer {

    fn hello() {
        let mut books = std::collections::HashMap::new();
        books.insert("AAAA".to_string(), "HHHH".to_string());
    }
}

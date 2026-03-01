mod front_of_house {
    mod hosting {
        use crate::{eat_at_restaurant, front_of_house};

        fn add_to_waitlist() {
            front_of_house::eat_at_restaurant();
        }

        fn seat_at_table() {}
    }

    mod serving {
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

    front_of_house::hosting::add_to_waitlist();
}

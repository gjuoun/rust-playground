mod front_of_house;

use front_of_house::hosting::add_to_waitlist;

pub use crate::front_of_house::*;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    hosting::add_to_waitlist();

    // absolute ref
    crate::front_of_house::hosting::add_to_waitlist();

    // relative ref
    front_of_house::hosting::add_to_waitlist();
}

mod front_of_house;

pub use crate::front_of_house::hostring;


pub fn eat_at_restaurant() {
    hostring::add_to_waitlist();
}
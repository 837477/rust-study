mod front_of_house;  // 선언부만 있고, 내용은 front_of_house.rs 파일에 있음

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
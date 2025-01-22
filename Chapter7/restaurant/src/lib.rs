mod front_of_house {
    pub mod hosting {  // 모듈을 공개하더라도, 내부 내용들까지도 공개되지 않음
        pub fn add_to_waitlist() {}  // 결국 이렇게 각각 내부 함수들을 공개해야 함

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house는 공개가 아니지만, eat_at_restaurant이 현재 lib.rs 크레이트에서 같이 상주하고 있는 형제 관계이기 때문에 접근 가능

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}




fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();  // super 키워드를 사용하여 부모 모듈로 올라갈 수 있음 (즉, ../ 같은 역할)
        // 현재 위치는 back_of_house 모듈이므로, super::deliver_order()는 back_of_house의 부모 크레이트인 lib.rs의 deliver_order() 함수를 호출
    }

    fn cook_order() {}
}



mod back_of_house {
    pub struct Breakfast {
        pub toast: String,  // 구조체의 필드는 기본적으로 비공개 (개별적으로 pub 키워드를 붙여 공개 가능)
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {  // pub로 된 생성자 함수 (연관 함수)를 무조건 제공 해주어야함. (Breakfast 구조체는 비공개 필드를 가지고 있기 때문)
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),  // seasonal_fruit 필드는 비공개이므로, 이렇게 초기화해주는 생성자(연관)함수가 필요함.
            }
        }
    }
}
pub fn eat_at_restaurant() {
    // 호밀 (Rye) 토스트를 곁들인 여름철 조식 주문하기
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 에러 발생, seasonal_fruit 필드는 비공개이므로 접근 불가
    // meal.seasonal_fruit = String::from("blueberries");
}



mod back_of_house {
    pub enum Appetizer {  // 열거형은 공개로 지정할 경우 모든 배리언트가 공개됨
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


// 관용적으로, hosting 모듈 내부에 있는 함수를 사용하려면 -> 부모 모듈까지만 use 구문을 사용하여 가져옴
// 이렇게 해야 확실하게 해당 함수가 로컬에 정의되어 있지 않음을 명백히 보여준다.
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();  // super 키워드를 사용하여 부모 모듈로 올라갈 수 있음
        // hosting::add_to_waitlist();  // 에러 발생, (use 구문과 다른 스코프에 존재)
    }
}



// 하지만, 열거형이나 구조체는 전체 경로를 사용하는 것이 관용적
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}



// 동일한 이름의 함수나 모듈을 가져올 때, as 키워드를 사용하여 로컬 이름을 변경할 수 있음
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
    // --생략--
}
fn function2() -> IoResult<()> {
    // --생략--
}



mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    // 일반적으로 use 키워드로 가져오면 비공개 모듈이지만, pub use로 가져오면 공개 모듈로 가져옴
    // 이렇게 사용 안했으면, front_of_house::hosting::add_to_waitlist(); 로 사용해야함
}



use std::cmp::Ordering;
use std::io;
// use 키워드를 사용하여 여러 개의 항목을 가져올 수 있음
use std::{cmp::Ordering, io};



use std::io;
use std::io::Write;
// use 키워드를 사용하여 같은 패키지의 여러 개의 항목을 가져올 수 있음
use std::io::{self, Write};



// glob 연산자(*)를 사용하여 패키지의 모든 공개 항목을 가져올 수 있음
use std::collections::*;

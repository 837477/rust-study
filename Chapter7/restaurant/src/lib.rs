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
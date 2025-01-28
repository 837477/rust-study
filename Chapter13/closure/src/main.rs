/*
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // user_preference가 Some()이면 그 값을 반환하고, None이면 클로저를 통하여 most_stocked()를 반환.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
*/


fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));

    // 여기서 컴파일 에러가 발생!
    // 처음 String::from("hello")를 넣었기 때문에, example_closure의 타입이 String으로 결정되었기 때문.
    let n = example_closure(5);

    // 클로저는 일반 함수들과 다르게 타입을 명시 안해도 됨.
    // 클로저는 통상적으로 짧고, 임의의 시나리오가 아닌 짧은 컨텍스트 내에서 사용됨.
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }   // 일반 함수
    let add_one_v2 = |x: u32| -> u32 { x + 1 };  // 클로저
    let add_one_v3 = |x|             { x + 1 };  // 클로저 (러스트의 타입 추론이 필요함)
    let add_one_v4 = |x|               x + 1  ;  // 클로저 (러스트의 타입 추론이 필요함)

    // 클로저는 세 가지 방식으로 값을 캡처
    // 불변 캡처(&T), 가변 캡처(&mut T), 소유권 캡처(T)

    // 불변 캡처
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // 가변 캡처
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut mut_borrows = || {
        list.push(4);
        println!("From closure: {:?}", list);
    };
    mut_borrows();
    println!("Before calling closure: {:?}", list);

    // 소유권 캡처
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let moving = move || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
}

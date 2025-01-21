#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --생략--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  // Quarter Variant에는 UsState 값이 포함됨
}

fn value_in_cents(coin: Coin) -> u8 {
    // match는 철저하다. (매칭 대상이 열거형인경우, 모든 패턴(Variant)을 다루지 않으면 컴파일 에러 발생)
    match coin {
        Coin::Penny => 1,  // match 키워드를 사용하여, 각 Variant에 대한 동작을 정의
        Coin::Nickel => {  // 중괄호를 사용하여 여러 줄의 코드를 사용할 수 있음
            println!("Lucky nickel!");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {  // state에 Quarter Variant에 포함된 UsState 값이 바인딩됨
            println!("State quarter from {:?}!", state);  // UsState 값에 대한 정보 출력
            25
        },
    }
}

// Option<T>도 열거형이므로, match 키워드를 사용하여 동작을 정의할 수 있음
fn plus_one(x: Option<i32>) -> Option<i32> {
    // match는 철저하다. (매칭 대상이 열거형인경우, 모든 패턴(Variant)을 다루지 않으면 컴파일 에러 발생)
    match x {
        None => None,  // Option의 None Variant에 대한 동작 정의
        Some(i) => Some(i + 1),  // Option의 Some Variant에 대한 동작 정의
    }
}

fn other_pattern() {
    let some_u8_value: u8 = 3;
    match some_u8_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        _ => (),  // _는 모든 패턴을 의미하며, 이 경우에는 다른 패턴에 해당하지 않을 때의 동작을 정의

        // 즉, 원래 match는 철저하기때문에 모든 열거형에 대하여 match 패턴을 다루어야하지만
        // _ (== other)를 사용하여, 나머지 패턴에 대한 동작을 정의할 수 있음
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value in cents: {}", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}", six);
    println!("None: {:?}", none);

    other_pattern();
}
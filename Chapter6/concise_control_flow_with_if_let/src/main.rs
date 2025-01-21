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

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 아래 if let으로도 사용 가능
    // 즉, if let은 한 패턴에 매칭될 때만 코드를 실행하고 다른 경우는 무시하는 match 문을 간단하게 표현하는 방법
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let 구문도, else를 사용할 수 있음 (즉, match의 _ 패턴과 동일)
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // 다음과 같음
    let mut count = 0;
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // Some 타입은 Rust에서 Option 열거형의 변형 중 하나이다.
    // Some은 특정한 값이 있을 때 사용하는 변형으로, Option<T> 타입에서 사용된다.
    // 여기서 T는 어떤 타입이든 될 수 있습니다.

    // 즉, Some은 Option 타입에 속한 변형이며, Option<T>가 어떤 자료형을 가지든 상관없이 Some 변형으로 값을 감쌀 수 있다.
    // - Option<u8>: Some(3u8) 또는 None
    // - Option<String>: Some(String::from("Hello")) 또는 None
    // - Option<f64>: Some(3.14) 또는 None

    // 따라서 Some이라고 하더라도 Option 타입일 경우에는 Some 변형에 감싸진 값의 타입이 무엇이든 Some을 통해 매칭할 수 있다.
    // match 구문에서 Some(value)라는 패턴을 사용하면, 그 값이 어떤 형태의 자료형이더라도 Option으로 감싸져 있는 경우에는 매칭이 가능합니다.
}

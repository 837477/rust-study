fn main() {
    another_function(5);
    print_labeled_measurement(42, 'C');

    // x = y = 6;  // 러스트에서는 이런 식의 연산은 지원하지 않는다.

    let y = {
        let x = 3;
        x + 1  // 표현식은 종결을 나타내는 세미콜론을 사용하지 않음. (세미콜론을 사용하면 구문으로 취급되어 반환값이 없는 것으로 간주됨.)
    };  // 중괄호로 만들어진 블록은 표현식이다. (블록의 마지막 표현식이 반환값이 된다.)
    println!("The value of y is: {y}");

    let x = five();  // five() 함수를 보면 5를 표현식으로 반환하고 있음. (즉, let x = 5; 와 동일)
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// 함수 매개변수에는 반드시 타입이 명시되어야 한다.
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 함수 본문의 마지막 표현식은 반환값이 된다.
// 반환값의 타입은 반드시 함수 시그니처에 명시되어야 한다.
fn five() -> u8 {
    5  // return 5; 와 동일
}

fn plus_one(x: i32) -> i32 {
    x + 1  // return x + 1; 와 동일
}
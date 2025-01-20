fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }

    /*
    // Rust의 조건문은 반드시 bool 타입이어야 한다.
    // 즉, 러스트는 Bool 타입이 아닌 값을 Bool 타입으로 자동 변환하지 않는다. (값이 있는 변수를 True라고 인식하지 않는다.)
    if number {
        println!("number was three");
    }
    */

    // Rust는 if else if 문 보다는 match 구문을 선호한다.
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    match number % 2 {
        0 => println!("짝수"),
        1 => println!("홀수"),
        _ => println!("둘 다 아님"),
    }

    // if 문은 표현식이기 때문에 다음과 같이 사용할 수 있다. (let 구문에서 if else 문 사용하기)
    // 단, 참 / 거짓의 타입이 서로 일치해야 한다.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
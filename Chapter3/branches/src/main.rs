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
    // 단, 참 / 거짓의 타입이 일치해야 한다.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // 반복문
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break 문 뒤에 표현식을 사용하면 값을 반환할 수 있다.
            break counter * 4;
        }
    };
    println!("The result is: {result}");

    // 루프 라벨
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
        println!("----------");
    }
    println!("End count = {count}");

    // while 문
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // 위 while 문을 loop 문으로 변경
    let mut number = 3;
    loop {
        if number == 0 {
            break;
        }
        println!("{number}!");
        number -= 1;
    }

    // for 문
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("----------");

    for element in a {
        println!("the value is: {element}");
    }
    println!("----------");

    for i in 0..=4 {
        println!("the value is: {}", a[i]);
    }
    println!("----------");

    for i in (0..=4).rev() {
        println!("the value is: {}", a[i]);
    }
    println!("----------");
}
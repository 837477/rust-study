fn main() {
    
    // 일반적인 러스트의 반복문
    loop {
        println!("again!");
        break;
    }

    // 반복문에서 값 반환
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break 문 뒤에 표현식을 사용하면 값을 반환할 수 있다.
        }
    };
    println!("The result is: {result}");

    // 루프 라벨을 사용하여 여러 반복문 사이의 모호성 제거
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
                break 'counting_up;  // 루프 라벨을 통하여 밖에 있는 loop를 탈출할 수 있다.
            }
            remaining -= 1;
        }

        count += 1;
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

    for element in a {
        println!("the value is: {element}");
    }

    for i in 0..=4 {
        println!("the value is: {}", a[i]);
    }

    for i in (0..=4).rev() {
        println!("the value is: {}", a[i]);
    }
}

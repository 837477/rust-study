// 이번 장에서는 여러 가지 패턴 매칭을 살펴볼 것이다.

fn main() {
    // [match]
    let x = 5;
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    // match 표현식에 대한 한 가지 제약사항은, 모든 경우의 수를 고려해야 한다.
    // 마지막 catchall(_) 사용하면 그 외의 경우를 처리할 수 있긴 하다.


    // [if let]
    // 주로 하나의 조건만 비교 / 매칭하고 싶을때, match보다 if let을 사용하여 더 간결하게 표현이 가능하다.
    let x = Some(5);
    if let Some(i) = x {
        println!("{}", i);
    } else {
        println!("None");
    }


    // [while let]
    // if let과 구조가 비슷하지만, 패턴이 계속 매칭되는 동안 반복한다.
    let mut v = vec![1, 2, 3, 4, 5];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }


    // [for loop]
    // for x in y에서 x가 패턴이다.
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // [let]
    // let 키워드를 사용하여 패턴 매칭을 할 수 있다.
    let (x, y, z) = (1, 2, 3);


    // [function parameter]
    // 함수의 매개변수로 패턴을 사용할 수 있다.
    fn foo((x, y): (i32, i32)) {
        println!("x: {}, y: {}", x, y);
    }


    // [리터럴 매칭]
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // [명명된 변수 매칭]
    // match 스코프 안에 변수는 외부 스코프의 변수를 가릴 수 있다.
    // some(y)는 match 스코프 내부의 새로운 변수이다.
    // 즉, y 바인딩은 Some 내부의 모든 값에 매칭되고 y는 그 값에 바인딩된다.
    // 따라서 y는 x안의 Some 값에 바인딩된다. (y = 5)
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);


    // [다중 패턴]
    // | 연산자를 사용하여 여러 패턴을 동시에 매칭할 수 있다.
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }


    // [범위 매칭]
    // ..= 연산자를 사용하여 범위를 매칭할 수 있다.
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }


    // [char 범위 매칭]
    // 문자도 범위 매칭할 수 있다.
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    // [구조체 해체]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    // let Point { x, y } = p; // 이렇게도 가능하다.
    assert_eq!(0, a);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }


    // [enum(열거형) 해체]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }


    // [중첩 구조체와 열거형 해체]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => ()
    }

    
    // [구조체와 튜플 해체]
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });


    // [_ 생략/무시 기호]
    // _를 사용하여 일부 값들을 무시할 수 있다. (컴파일 경고를 방지할 수 있다.)
    // EX1)
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
    // EX2)
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    // EX3)
    let _x = 5;
    let y = 10;  // y만 컴파일시 unused 경고가 발생
    
    // _와 _<변수> 는 약간의 차이가 있다.
    // _는 바인딩을 전혀 하지 않고 무시하지만, _<변수>는 바인딩을 진행
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {  // 바인딩 됨. (소유권 이전)
        println!("found a string");
    }
    println!("{:?}", s);  // 컴파일 에러 발생

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {  // 바인딩 안됨. (소유권 이전 안됨)
        println!("found a string");
    }
    println!("{:?}", s);  // 컴파일 에러 발생 X


    // [.. 생략 기호]
    // ..을 사용하여 나머지 값들을 무시할 수 있다.
    // EX1)
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }
    // EX2)
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last)
        }
    }


    // [매치 가드]
    // 매치 가드는 match 분기에 추가 조건을 추가할 수 있다.
    // 매치 가드는 if 키워드와 함께 사용된다.
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }


    // [복잡한 패턴]
    // 패턴은 다양한 방법으로 조합할 수 있다.
    // EX1)
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }


    // [@ 바인딩]
    // @ 바인딩을 사용하여 패턴을 변수에 바인딩할 수 있다.
    // EX1)
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}

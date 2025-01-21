struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    // 불변 인스턴스 생성
    let user1 = User {
        active: true,
        email: String::from("user1@example.com"),  // 정의한 필드 순서가 맞을 필요 없음.
        username: String::from("user1"),
        sign_in_count: 1
    };
    println!("User1's email: {:?}", user1.email);

    // 가변 인스턴스 생성 (전체 가변성 / 일부 필드만 가변으로 만들 수 없음)
    let mut user2 = User {
        active: false,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 1
    };
    user2.email = String::from("user22@example.com");
    println!("User2's email: {:?}", user2.email);

    // Builder 패턴을 사용하여 인스턴스 생성
    let user3 = build_user(
        String::from("user3"),
        String::from("user3@example.com")
    );
    println!("User3's email: {:?}", user3.email);

    // 기존 인스턴스를 이용해, 새 인스턴스 생성 (1)
    // 단, active와 sign_in_count은 Static Dispatch 방식으로 복사됨
    let user4 = User {
        email: String::from("user4@example.com"),
        username: String::from("user4"),
        ..user3  // user3의 나머지 필드들을 사용 (주의, 제일 끝에 위치해야 함)
    };
    println!("User4's active: {:?}", user4.active);

    // 기존 인스턴스를 이용해, 새 인스턴스 생성 (2)
    // 단, eamil과 username은 "이동" 되어, 기존의 user2의 email과 username은 사용할 수 없음
    let user5 = User {
        active: true,
        sign_in_count: 1,
        ..user2  // user2의 나머지 필드들을 사용 (주의, 제일 끝에 위치해야 함)
    };
    // println!("User2's email: {:?}", user2.email);  // 컴파일 에러 발생 (user2의 email은 이동되었으므로 사용 불가)

    // Tuple 구조체 인스턴스 생성
    // 내부 타입이 동일하더라도, 서로 다른 타입의 구조체로 인식됨
    // 즉, Color 타입을 매개변수로 받는 함수에 Point 타입을 인수로 넘겨주는 등의 행위는 불가능
    struct Color(i32, i32, i32);  // 튜플 구조체는 필드 이름은 없음, 필드 타입만 존재
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 필드가 없는 유닛 구조체 인스턴스 생성
    struct UnitStruct;
    let unit = UnitStruct;
}


fn build_user(username: String, email: String) -> User {
    User {
        email,  // 필드명과 변수명이 같을 경우, email: email와 같이 쓸 필요 없음. (축약 문법)
        username,  // 필드명과 변수명이 같을 경우, username: username와 같이 쓸 필요 없음. (축약 문법)
        active: true,
        sign_in_count: 1,
    }
}
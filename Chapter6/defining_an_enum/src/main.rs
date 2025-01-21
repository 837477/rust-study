enum IpAddrKind {
    V4,  // Variant
    V6,
}
enum IpAddrKind2 {
    V4(String),  // 각 Variant에 데이터를 저장할 수 있음
    V6(String),
}
enum IpAddrKind3 {
    V4(u8, u8, u8, u8),  // 각 Variant에 다양한 타입의 데이터를 저장할 수 있음
    V6(String),
}
enum Message {
    Quit,  // 데이터가 전형 없는 Variant
    Move { x: i32, y: i32 },  // 구조체처럼 이름 있는 필드를 갖는 Variant
    Write(String),  // 하나의 String 데이터를 가지는 Variant
    ChangeColor(i32, i32, i32),  // 세 개의 i32 데이터를 갖는 Variant
    
    // 즉, 열거형은 다양한 타입의 데이터를 가질 수 있음
    // 이는 구조체와는 다르게, 다양한 타입의 데이터를 하나의 타입으로 표현할 수 있음
    
    /*
    // 구조체로 표현하면 다음과 같음
    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);
    */
}
impl Message {
    fn call(&self) {
        // 메서드 정의
    }
}

struct IpAddr {
    kind: IpAddrKind,  // 열거형을 필드로 가지는 구조체
    address: String,
}


fn main() {
    // 열거형을 정의할 때 식별자로 네임스페이스가 생성됨
    // 즉, V4와 V6는 IpAddrKind의 Variant이므로, IpAddrKind 타입이라는 것을 알 수 있음.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // 열거형을 구조체의 일부로 사용
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    
    // 열거형 자체를 조금 더 간결하게 사용
    let home2 = IpAddrKind2::V4(String::from("127.0.0.2"));
    
    // 열거형의 Variant에 다양한 타입의 데이터를 저장
    let home3 = IpAddrKind3::V4(127, 0, 0, 3);
    let home4 = IpAddrKind3::V6(String::from("::1"));

    // 열거형의 매서드 호출
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 열거형
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;  // None은 컴파일러가 타입을 추론할 수 없기 때문에 타입을 명시해주어야 함 

    // Option<T> 와 T는 서로 다른 타입
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;  // 컴파일 에러 발생 (서로 다른 타입이므로 연산 불가)

    // 즉, Null로 인하여 발생하는 다양한 에러를 사전에 방지 가능
    // 위 연산을 수행하기 위해서는, Option<T>를 T로 변환해주어야 함
    // 러스트 입장에서는 Option<T>를 T로 변환해야만, 더이상 Null이 존재하지 않음을 보장할 수 있음
}


fn route(ip_type: IpAddrKind) {
    // 이렇게 열거형을 사용하면, 다양한 타입을 하나의 타입으로 표현할 수 있음
    // 즉, 해당 함수의 매개변수로는 IpAddrKind 타입으로 V4와 V6 두 가지 Variant를 가질 수 있음
}
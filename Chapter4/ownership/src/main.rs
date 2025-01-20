// 러스트의 소유권 규칙
// 1. 러스트에서, 가각의 값은 소유자(owner)가 정해져 있다.
// 2. 한 값의 소유자는 동시에 여럿이 존재할 수 없다.
// 3. 소유자가 스코프 밖으로 벗어날 때, 값이 버려진다. (drop 함수가 호출된다.)

/*
fn main() 
{  // s는 아직 선언되지 않음.
    let s = "hello";  // 이 지점부터 s는 유효하다.
}  // 이 스코프는 끝났고, s는 더 이상 유효하지 않다.
*/



/*
fn main() {
    // 소유권 설명을 위한 String 타입
    // 기존의 문자열 리터럴은 불변이지만, String 타입은 가변이다.
    
    let s = String::from("hello");
    s.push_str(", world!");  // push_str()은 문자열을 추가하는 메소드
    println!("{s}");

    // 문자열 리터럴: 컴파일 타임에 알 수 있으므로, 텍스트가 최종 실행파일에 하드코딩됨. (즉, 빠르고 효율적이지만 불변이다.)
    // String 타입: 런타임에 크기가 변할 수 있으므로, 힙에 할당되고, 메모리를 해제하는 작업이 필요하다. (즉, 유연하지만 느리고 비용이 많이 든다.)
}
*/



/*
fn main() {
    // 기존에 가비지 컬렉터가 없는 언어는, 프로그래머가 직접 할당과 해제를 신경써주어야 한다.
    // 하지만 이러한 방식은 잦은 실수로 인해 버그가 발생할 수 있다.
    // 따라서, 러스트는 변수가 자신이 소속된 스코프를 벗어나는 순간 자동으로 메모리를 해제하는 방식(소유권)을 사용하여 해결.
    
    let s = String::from("hello");  // s는 이 시점부터 유효
}  // 이 스코프가 끝나면 s는 더 이상 유효하지 않음. (drop 함수가 호출되어 메모리가 해제됨.)
*/



/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1에는 String 데이터가 저장됨. ("hello"가 할당되어있는 곳의 주소를 가리키는 포인터 / 길이 / 용량)
    // s2에는 s1의 데이터가 복사되어 저장됨. (즉, "hello"가 할당되어있는 곳의 주소는 같은 곳을 가리킴)
    // 따라서 이 시점부터는 s1은 더 이상 유효하지 않음.
    // 이걸 러스트에서는 데이터의 이동(move)이라고 부름. (얕은 복사 X)

    // println!("{s1}");  // 컴파일 에러 발생
    
    println!("{s2}");  // s2는 여전히 유효함.

    // 추가적으로, 러스트에서는 절대 깊은 복사(deep copy)를 하지 않음.
    // 만약, 힙 데이터 영역까지 복사하고 싶을 때에는 clone() 메소드를 사용.
    let s3 = s2.clone();
    println!("{s3}");

    // 정수형은 clone()을 호출하지 않았는데도 계속해서 사용할 수 있는 이유는, 정수형은 스택에 저장되기 때문.
    let x = 5;
    let y = x;
    println!("{x} {y}");
}
*/



/*
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s의 값이 함수로 이동함.
    // println!("{s}");  // 컴파일 에러 발생 (s는 takes_ownership() 함수의 스코프가 종료될때 메모리가 해제됨. == 더 이상 유효하지 않음.)

    let x = 5;
    makes_copy(x);  // x의 값이 함수로 복사됨.
    println!("{x}");  // x는 여전히 유효함. (정수형은 스택에 저장되기 때문에 이동이 아닌 복사가 일어남.)
}
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
*/



fn main() {
    let s1 = gives_ownership();  // gives_ownership() 함수의 반환값이 s1으로 이동함.
    let s2 = String::from("hello");  // s2가 생성됨.
    let s3 = takes_and_gives_back(s2);  // s2의 값이 함수로 이동되고, takes_and_gives_back() 함수에서 다시 반환해주어서 s3로 이동함.

    // 소유권을 다시 반납해주는 방식
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);  // s1의 소유권이 calculate_length() 함수로 이동되고, 반환값으로 s2와 len이 반환됨.

    // 이런 방식으로 모든 함수가 소유권을 가졌다가 다시 반납해주는 것은 조금 번거롭다.
    // 즉, 함수의 목적은 길이를 구하는 것인데 다시 소유권을 반납해주는 것은 부수적인 일이다.
    // 따라서 러스트에서는 참조자(reference)를 사용하여 이 문제를 해결한다. (다음 references_and_borrowing 디렉토리 참고)
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string  // some_string이 반환되어 s1으로 이동됨.
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string이 반환되어 s3으로 이동됨.
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // len() 메소드는 문자열의 길이를 반환함.
    (s, length)  // s와 length가 반환됨.
}
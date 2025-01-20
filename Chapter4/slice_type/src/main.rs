// Slice는 참조자의 일종으로서 소유권을 가지지 않음.


/*
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();  // 문자열을 바이트 배열로 변환
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];  // 공백 문자를 만나면, 이전까지의 문자열을 반환
        }
    }
    &s[..]  // 공백 문자가 없다면, 전체 문자열을 반환
}
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();  // 컴파일 에러 발생 (word는 s의 일부분을 참조하고 있기 때문에, s를 변경할 수 없음.)
    // clear() 함수는 문자열을 비우는 함수이므로, 내부적으로 가변 참조자를 사용함.
    
    // 추가적으로, println! 에서 word의 참조자를 사용하므로, 이 불변 참조자는 이 지점까지 계속 활성화 되어있어야 함.
    println!("The first word is: {word}");
}
*/



/*
fn main() {
    let s = String::from("hello world");

    // 다음은 슬라이싱된 String의 일부분을 가리키는 참조자이다.
    let hello = &s[0..5];  // hello
    let world = &s[6..11];  // world

    let slice = $s[..2];  // Start Index가 0부터 시작하면 생략 가능
    let slice = $s[3..];  // End Index가 문자열의 길이와 같다면 생략 가능
    let slice = $s[..];  // Start Index와 End Index가 모두 생략되면 전체 문자열을 나타냄
}
*/



/*
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();  // 문자열을 바이트 배열로 변환
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];  // 공백 문자를 만나면, 이전까지의 문자열을 반환
        }
    }
    &s[..]  // 공백 문자가 없다면, 전체 문자열을 반환
}
fn main() {
    let my_string = String::from("hello world");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스[..]와 동일한 `String`의 참조자에 대해서도 작동합니다
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // (중요) 문자열 리터럴은 곧 문자열 슬라이스이므로, 아래의 코드도 슬라이스 문법 없이 작동합니다!
    let word = first_word(my_string_literal);
}
*/



fn main() {
    // 그 밖에, 슬라이스는 배열의 일부분을 참조하는데도 사용할 수 있다.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // [2, 3]
    let slice = &a[1..=3];  // [2, 3, 4]
    let slice = &a[..3];  // [1, 2, 3]
    let slice = &a[1..];  // [2, 3, 4, 5]
    let slice = &a[..];  // [1, 2, 3, 4, 5]
}
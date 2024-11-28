fn main() {
    let mut s = String::from("hello world");

    let hello = first_word_index(&s);
    println!("The first word is: {}", hello);

    s.clear(); // String을 비움

    // hello는 여전히 유효하지만, s는 이제 빈 문자열이므로 hello는 더 이상 의미가 없음.
    println!("s = {}", s);

    //////////////////////////////////////////////////////////////////////////////

    let s = String::from("hello world");

    let he = &s[0..2];
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("he: {}, hello: {}, world: {}", he, hello, world);

    let a = &s[..3];
    let b = &s[3..];
    let c = &s[..];
    println!("a: {}, b: {}, c: {}", a, b, c);

    //////////////////////////////////////////////////////////////////////////////

    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear(); // 에러!
    // clear 함수는 String의 길이를 변경해야 하니 가변 참조자가 필요
    // clear 호출 이후 println!는 word의 참조자를 사용하므로, 이 불변 참조자는 이 지점까지 계속 활성화되어 있어야 함.
    println!("the first word is: {}", word);

    //////////////////////////////////////////////////////////////////////////////
    
    let my_string = String::from("hello world");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의 참조자에 대해서도 작동.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로, 아래의 코드도 슬라이스 문법 없이 작동.
    let word = first_word(my_string_literal);

    println!("the first word is: {}", word);
}


fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
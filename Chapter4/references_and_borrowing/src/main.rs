// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);  // 참조자로 s1을 넘김 (대여)

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }  // s는 여기서 스코프 밖으로 벗어났지만, 아무 일도 일어나지 않음 (소유X / 대여중)


// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("{}", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


fn main() {
    let mut s = String::from("hello");

    // 1. 불변 참조를 생성하고 사용
    let r1 = &s;
    println!("Before modification: {}", r1); // "hello"

    // 2. 불변 참조의 스코프 종료 (여기까지 사용됨)

    // 3. 원본을 수정
    s.push_str(" world");

    // 4. 수정 후 다시 불변 참조 생성
    let r2 = &s;
    println!("After modification: {}", r2); // "hello world"

    // 만약 수정 후 기존의 r1을 그대로 출력하려고 하면 오류가 발생
    // 이유는 이미 불변 참조 r1이 아래에 존재하기 때문에, push_str() 메소드 안에서 가변 참조를 생성할 수 없음
}


// Dangling Reference
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle은 String의 참조자를 반환

    let s = String::from("hello"); // s는 새로운 String

    &s // String s의 참조자를 반환
} // 여기서 s는 스코프 밖으로 벗어나고 버려짐. (즉, 해당 메모리는 해제됨)
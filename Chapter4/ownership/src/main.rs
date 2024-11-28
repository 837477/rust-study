// fn main() {
//     let s = String::from("hello");  // s가 스코프 안으로 들어옴

//     takes_ownership(s);             // s의 값이 함수로 이동
//                                     // 여기서부터 더 이상 유효하지 않음

//     let x = 5;                      // x가 스코프 안으로 들어옴

//     makes_copy(x);                  // x가 함수로 이동되지만, i32는 Copy이므로 앞으로 계속 x를 사용해도 무관

// } // 여기서 x와 s가 스코프 밖으로 벗어나지만, s는 이동되었으므로 아무 일도 일어나지 않음.

// fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옴
//     println!("{}", some_string);
// } // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출 (메모리 해제)

// fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옴
//     println!("{}", some_integer);
// } // 여기서 some_integer가 스코프 밖으로 벗어나지만, 아무 일도 일어나지 않음 (i32는 Copy이므로)



fn main() {
    let _s1 = gives_ownership();         // gives_ownership의 반환 값을 s1로 이동

    let s2 = String::from("hello");      // s2가 스코프 안으로 들어옴

    let _s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데, 이 함수에서 반환 값을 s3로 이동


} // 여기서 s3가 스코프 밖으로 벗어나면서 버려짐
  // s2는 이동되어서 아무 일도 일어나지 않음
  // s1은 스코브 밖으로 벗어나면서 버려짐

fn gives_ownership() -> String {         
    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옴
    some_string                              // some_string이 반환되고 호출자 함수 쪽으로 이동
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동
}
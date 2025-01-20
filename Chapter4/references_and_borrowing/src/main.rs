// 전 실습에서, 함수가 소유권을 가졌다가 그 이후에도 계속 사용하고 싶어서 다시 반납해주는 것은 조금 번거롭다.
// 참조라를 통하여 해결 가능.


/*
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1은 s1의 참조자를 의미함.
    println!("The length of '{s1}' is {len}");
}
fn calculate_length(s: &String) -> usize {  // 매개변수에도 참조자를 명시.
    s.len()  // s는 String의 참조자이기 때문에, s.len()으로 String의 길이를 반환할 수 있다.
}  // 스코프가 끝나더라도 s는 소유권을 가지고 있지 않기 때문에 메모리가 해제되지 않는다. (borrowing)
*/



/*
fn main() {
    let s = String::from("hello");

    change(&s);  // s의 참조자를 change() 함수에 전달함.  (Borrowing)
}
fn change(some_string: &String) {
    some_string.push_str(", world");  // 컴파일 에러 발생 (참조자는 기본적으로 불변이기 때문에 값을 변경할 수 없음.)
}  // some_string의 소유권이 없기 때문에, 스코프가 끝나도 메모리가 해제되지 않음.
*/



/*
fn main() {
    let mut s = String::from("hello");

    change(&mut s);  // s의 가변 참조자를 change() 함수에 전달함.  (Mutable Borrowing)

    println!("{s}");

    // (주의) 어떤 값에 대한 가변 참조자가 있다면, 그 값에 대한 참조자는 더 이상 만들 수 없다. (딱 하나만 만들 수 있다.)
    let r1 = &mut s;
    let r2 = &mut s;  // 컴파일 에러 발생 (이미 가변 참조자가 있기 때문에, 또 다른 가변 참조자를 만들 수 없음.)
    // 즉, 첫 번째 가변 대여는 r1한테 있어서 다음의 println!에서 사용될때까지 남아있어야 한다.
    println!("{}, {}", r1, r2);
    

    // 만약 이러한 데이터 경합을 피하려면 다음과 같이 사용할 수 있음.
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }  // r1의 스코프가 끝나면, 다음 가변 참조자를 만들 수 있음.
    let r2 = &mut s;

    
    // (주의) 가변 참조자와 불변 참조자를 혼용할 때에도 컴파일 에러가 발생함.
    // 불변 참조자 입장에서는, 사용 중인 값이 중간에 변경되리라 예상하지 않을테니.
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // 컴파일 에러 발생 (가변 참조자와 불변 참조자를 혼용할 수 없음.)
    println!("{r1}, {r2}, {r3}");


    // 만약, 불변 참조자를 사용한 후라면 가변참조자를 사용할 수 있다.
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");
    let r3 = &mut s;
    println!("{r3}");

}
fn change(some_string: &mut String) {
    some_string.push_str(", world");  // 가변 참조자를 통하여 값을 변경할 수 있음.
}  // some_string의 소유권이 없기 때문에, 스코프가 끝나도 메모리가 해제되지 않음. (borrowing)
*/



fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // s의 참조자를 반환함.
}  // 하지만 s는 이 시점에서 메모리가 해제되어 더 이상 유효하지 않음. (dangle)
/*
// 일반적인 함수를 사용하여 사각형의 면적을 계산하는 프로그램
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)  // 두 인수가 서로 연관성이 있는지는 한눈에 알기 어려움
    );
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/



/*
// 튜플을 사용하여 사각형의 면적을 계산하는 프로그램
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)  // 인수가 서로 연관된 값이라는 것을 알기 쉽게 표현 (한 개만 넘겨도 됨)
    );
}
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1  // 대신 인덱스로 접근해야 하므로 가독성이 떨어짐 (어떤게 width고 height인지 알기 어려움)
}
*/


/*
// 구조체를 사용하여 사각형의 면적을 계산하는 프로그램
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)  // 인수가 서로 연관된 값이라는 것을 알기 쉽게 표현 (한 개만 넘겨도 됨)
        // 불변참조자를 사용하여, rect1의 소유권을 유지하면서도 값을 빌림
        // 소유권을 이동시켜도 사실 상관 없음 (이후 사용하지 않으므로)
    );
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height  // 필드명으로 접근 가능하여 가독성이 좋음
}
*/



// 트레이트 파생으로 기능 추가하기
#[derive(Debug)]  // Debug 트레이트를 파생하여, 디버깅 시 사용할 수 있는 문자열로 변환
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // println!("rect1 is {}", rect1);  // 컴파일 에러 발생
    // println! 매크로는 {} 플레이스홀더에 대해 Display 트레이트를 구현한 타입만 사용 가능
    // Rectangle 구조체는 Display 트레이트를 구현하지 않았으므로, 컴파일 에러 발생

    println!("rect1 is {:?}", rect1);  // Debug 트레이트를 파생했으므로, {:?} 플레이스홀더 사용 가능
    println!("rect1 is {:#?}", rect1);  // {:#?} 플레이스홀더는 보기 좋게 출력
    // {:?} 플레이스홀더는 Debug 트레이트를 구현한 타입만 사용 가능

    // dbg! 매크로를 사용하여, 디버깅 시점에 변수의 값을 출력할 수 있음
    let rect2 = Rectangle {
        width: dbg!(30 * 2),  // dbg! 매크로는 표현식의 소유권을 가져와서, 사용(출력)한 뒤 그대로 반환
        height: 50
    };
    dbg!(&rect2);
}
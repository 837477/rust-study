#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {  // impl == implementation(구현)
    // 구조체에 대한 메서드 정의
    fn area(&self) -> u32 {  // 매소드의 첫 번째 매개변수는 항상 self
        // &self == self: &self (Rust 컴파일러가 자동으로 변환해줌)
        // 가변참조도 가능 (&mut self)
        self.width * self.height
    }

    // 필드와 매서드명을 둘다 중복되어 지을 수 있다.
    fn width(&self) -> bool {
        self.width > 0
        // 사용하는 곳에서 다음과 같이 인식함.
        // ()를 붙이면, 매서드로 인식
        // ()를 붙이지 않으면, 필드로 인식
    }

    // 다중 매개변수를 받는 매서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 연관 함수 (self 매개변수를 갖지 않는 함수)
    // 연관 함수는 보통 생성자로 사용 (새 인스턴스를 반환하는 생성자)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("width is {}", rect1.width());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 연관 함수 호출
    let sq = Rectangle::square(3);
    println!("sq is {:?}", sq);
}
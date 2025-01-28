/*
enum List {
    // List 타입이 알려진 크기를 가지고 있지 않음. (컴파일 X)
    // 즉, 러스트는 컴파일 할 시점에 List 타입의 크기를 알 수 없음
    Cons(i32, List),
    Nil,
}
*/
enum List {
    // Box<T>가 포인터이기 때문에, 러스트는 언제나 Box<T>가 필요로 하는 공간이 얼마인지 알고 있다.
    // 즉, Box<T>는 항상 고정된 크기를 가지고 있음 (주소를 담을 수 있는 크기)
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Box를 사용하여 힙에 i32 값을 저장
    // 스코프가 벗어날때, Box는 힙에 할당된 메모리를 해제
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

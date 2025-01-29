// Rc<T>와 다르게, RefCell<T>는 런타임에 빌림 규칙을 적용한다.
// 참조자와 Box<T>를 이용할때는 컴파일 타임에 빌림 규칙을 적용한다.
// Rc<T>와 마찬가지로 RefCell<T>도 싱글스레드 시나리오 내에서만 사용 가능하다./
// Rc<T>는 복수 소유자, RefCell<T> / Box<T>는 단일 소유자이다.
// RefCell<T>이 런타임에 검사되는 가변 대여를 허용하기 때문에, RefCell<T>이 불변일 때라도 RefCell<T>의 내부 값을 변경할 수 있다.

/*
fn main() {
    // 일반적으로는 다음과 같이 불변값을 가지고 있을때 이걸 가변으로 빌려올 수 없다. (컴파일 에러)
    let x = 5;
    let y = &mut x;
}
*/

// Rc<T>와 RefCell<T>를 조합하여 가변 데이터의 복수 소유자 만들기
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 먼저 Rc<RefCell<i32>>를 이용하여 가변 데이터를 가지는 List를 생성한다.
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // Rc<T> 실습때와 마찬가지로, b와 c는 a를 참조하게 만든다.
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // value의 값을 변경해보자.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
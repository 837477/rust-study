/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // 컴파일 에러 코드 (소유권 이동 이슈)
    // a의 소유권이 b로 이동되었기에, c를 생성할 때 a를 사용할 수 없다.
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // Rc<T>를 사용하여 참조 카운팅을 사용하여 해결
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // 여기서 clone() 은 메모리 복사가 아닌 참조 카운팅을 증가시키는 것이다.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // Rc<T>.strong_count() 함수를 사용하여 참조 카운팅을 확인할 수 있다.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
use std::ops::Deref;

struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = &x;  // x의 주소를 y에 저장

    assert_eq!(5, x);
    assert_eq!(5, *y);  // y가 가리키는 주소에 저장된 값을 가져옴

    let z = Box::new(x);  // x가 가리키고 있는 값을 힙에 복사
    assert_eq!(5, *z);  // z가 가리키는 주소에 저장된 값을 가져옴

    let w = MyBox(x);  // x가 가리키고 있는 값을 힙에 복사
    assert_eq!(5, *w);  // w가 가리키는 주소에 저장된 값을 가져옴

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &m은 MyBox<string>에 대한 참조자. (&m == &MyBox<String>)
    // hello() 함수는 &str에 대한 참조자를 받음
    // 하지만 deref 트레이트의 반환값이 &self.0 이므로, String에 대한 참조자가 반환됨.
    // 아직도 &String이지만, String도 deref 트레이트가 구현되어있고 반환값이 &str 이므로 &str로 변환됨.

    //만약 역참조 강제 변환이 구현되어 있지 않았따면, 아래와 같이 작성해야함
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    // *m은 MyBox<String>에 대한 참조자. (*m == *MyBox<String>)
    // 즉, String에 대한 참조자가 반환됨.
    // &String[..] 은 String의 슬라이스를 반환함.
}
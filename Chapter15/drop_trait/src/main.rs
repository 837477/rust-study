struct CustomSmartPointer {
    data: String,
}

// Drop 트레이트는, 기본적으로 프렐루드에 포함되어있다.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // 변수가 만들어진 역순으로 drop 메소드가 호출된다. (역순으로 버려짐)
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // 러스트는 표준라이브러리가 제공하는 std::mem::drop 함수를 통해, 스코프가 끝나기 전에 수동으로 drop을 호출할 수 있다.
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    
    // 단, 이렇게 수동으로 drop을 호출하면, 스코프가 끝나기 전에 drop이 호출되어, 변수가 두번 drop되는 문제가 발생할 수 있다.
    // 즉, 러스트는 직접 drop을 명시적으로 호출하는 것을 허락하지 않는다. (컴파일 에러 발생)
    // c.drop();

    // 따라서, std::mem::drop 함수를 사용하여 drop을 호출해야 한다.
    // std::mem::drop 함수는 Drop 트레이트에 있는 drop 메서드와 다르다.
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

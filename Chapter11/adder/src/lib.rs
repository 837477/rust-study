#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

// 러스트는 비공개 함수도 테스트할 수 있다.
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// cargo test와 같이, 테스트를 실행할 때만 컴파일됨.
// 유닛테스트는 일반 코드와 같은 파일에 위치하기 때문에, cfg(test) 어노테이션을 사용하여 테스트 코드가 컴파일되지 않도록 한다.
// 반면에, 통합테스트는 별도의 디렉터리에 위치하기때문에 cfg(test) 어노테이션을 사용하지 않는다.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        
        // 추가 인수로 메시지를 출력할 수 있다.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    #[ignore]  // 테스트를 무시하려면 ignore 속성을 사용한다.
    fn greater_than_100() {
        // should_panic은 내부에서 패닉이 발생해야 테스트가 성공한다.
        // should_panic 속성의 expected 매개변수값이 Guess::new 함수에서 발생한 패닉 메시지 문자열의 일부와 일치하면 테스트가 성공한다.
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}



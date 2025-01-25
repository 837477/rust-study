// Lifetime의 주목적은 댕글링 참조를 방지하는 것이다.

fn main() {
    let r;                // ---------+-- 'a (라이프타임)
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
// 러스트는 대여 검사기(borrow checker)를 사용하여 런타임에 댕글링 참조를 방지한다.
//'a 라이프타임을 갖는 r이, 'a 라이프타임보다 짧은 'b 라이프 타임을 갖는 x를 참조하려고 하기 때문에 컴파일 에러가 발생한다.


fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
// 이 경우에는 'a 라이프타임을 갖는 r이, 'b 라이프타임을 갖는 x를 참조하고 있기 때문에 컴파일 에러가 발생하지 않는다.


// 함수에서의 제네릭 라이프타임 (다음 코드들은 에러 발생)
// 러스트 입장에서는, x가 반환될지 y가 반환될지 알 수 없기 때문에 컴파일 에러가 발생한다.
// 즉, 둘 중 어느 값이 더 오래 살아남을지 알 수 없다.
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 라이프타임 명시 문법
// &i32         // 참조자
// &'a i32      // 명시적인 라이프타임이 있는 참조자
// &'a mut i32  // 명시적인 라이프타임이 있는 가변 참조자


// 함수 시그니처에서 라이프타임 명시
// 두 매개변수에 명시된 라이프타임이 유효하는 동안에는 반환된 참조자도 유효하다는 것을 의미한다.
// 즉, 함수는 두 매개변수를 갖고 둘 다 적어도 라이프타임 'a만큼 살아있는 &str이며,
// 반환하는 &str도 라이프타임 'a만큼 살아있다는 정보를 알려줌
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 사실 longest 함수는 x와 y가 얼마나 오래 살지 정확히 알 필요는 없고,
    // 이 시그니처를 만족하는 어떤 스코프를 'a로 대체할 수 있다는 점만 알면 된다.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 이 코드는 정상적으로 실행된다.
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// 이 코드는 에러가 발생한다.
// 함수 매개변수와 반환 값 모두 동일한 라이프타임 매개변수 'a를 명시했지만,
// string1과 string2의 라이프타임이 다르기 때문에 에러가 발생한다.
// 즉, 다른 스코프에 위치한다는 것을 러스트가 알 수 있어서 에러가 발생한다.
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}


// 라이프타임 매개변수 명시의 필요성은 함수 사용에 따라 다르다.
// 항상 x만 반환하므로, x의 라이프타임만 명시해도 충분하다.
fn longest<'a> (x: &'a str, y: &str) -> &'a str {
    x
}

// 그리고 추가적으로, 반환타입의 라이프타임 매개변수는, 반드시 매개변수 중 하나이어야 한다.
// 그 이유는 다음 코드와 같이, 함수 내부에서 만들어진 참조자를 반환한다면 함수가 끝나는 시점에 스코프를 벗어나므로 댕글링 참조가 될 것이기 때문.
// 사실 어떻게보면, 반환 타입이 "참.조.자"인데 스코프 안에서 탄생한 참조자를 반환하면 댕글링 참조가 되는 것은 당연한 것이다.
fn longest<'a> (x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}


// 구조체 정의에서 라이프타임 명시
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // novel 변수가 소유하는 String 첫 문장에 대한 참조를 first_sentence가 가지며,
    // 이 참조를 part 필드에 저장하는 구조체 i를 생성한다.
    // novel 데이터는 ImportantExcerpt 인스턴스가 생성되기 전부터 존재
    // 즉, 이러한 구조체 라이프타임은 내부 필드가 보관하는 참조자의 라이프타임보다 오래살 수 없다.
}


// 라이프타임 생략
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// 사실 러스트의 초기버전까지만 해도, 모든 참조자는 라이프타임을 가지며 참조자를 사용하는 함수나 구조체는 라이프타임 매개변수를 명시해야했음.
// 하지만 러스트의 역사를 되돌아보면, 러슽트 프로그래머들이 특정한 상황에서 똑같은 라이프타임 명시를 반복하는 것이 번거롭다고 생각했음.
// 따라서 러스트는 라이프타임 생략 규칙을 도입하였다.

// 먼저 함수나 매서드 매개변수의 라이프타임은 "입력 라이프타임",
// 반환 값의 라이프타임은 "출력 라이프타임"이라고 부른다.

// 러스트는 라이프타임을 생략할 수 있는 규칙을 가지고 있으며, 이 규칙은 총 세개이다.
// (이 세가지 규칙을 모두 적용했음에도 라이프타임을 알 수 없다면, 러스트는 컴파일 에러를 발생시킨다.)
// 1. 참조자인 매개변수 각각에 라이프타임을 할당한다.
// 2. 입력 라이프타임이 하나인 경우, 그 라이프타임을 모든 출력 라이프타임에 할당한다.
// 3. 입력 라이프타임이 여러 개인데, 그중 하나가 &self 또는 &mut self인 경우, self의 라이프타임을 모든 출력 라이프타임에 할당한다.

// EX) fn first_word(s: &str) -> &str {}
// 위 함수는 입력 라이프타임이 하나이므로, 출력 라이프타임에도 할당된다. (규칙 1 + 2 == 컴파일 됨)

// EX) fn longest(x: &str, y: &str) -> &str {}
// 위 함수는 입력 라이프타임이 두개이므로, 출력 라이프타임을 알 수 없다.
// &self, &mut self 도 없어서 규칙 3도 적용이 X (컴파일 에러 발생)


// 메서드 정의에서 라이프타임 명시
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}  // (규칙1) 적용

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
} // (규칙3) 적용


// 정적 라이프타임
// 'static 라이프타임은 프로그램 전체 기간동안 유효하다.
fn main() {
    let s: &'static str = "I have a static lifetime.";
    // 문자열 리터럴은 프로그램이 종료될 때까지 메모리에 남아있기 때문에 'static 라이프타임을 가진다.
}
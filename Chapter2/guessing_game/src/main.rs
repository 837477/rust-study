use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

// 다음과 같이도 사용 가능
// use std::io;
// use std::io::Write;
// 기본적으로 러스트는 모든 프로그램의 스코프로 가져오는 표준 라이브러리에 정의된 아이템 집합을 가지고 있다. (이 집합을 prelude 라고 부른다.)

fn main() {
    // 랜덤 숫자 생성
    let secret_number = rand::thread_rng().gen_range(1..101);  // (1..=100)로도 사용 가능

    // 반복
    loop {
        // 개행 있는 출력
        println!("Guess the number!");

        // 개행 없는 출력
        print!("Please input your guess: ");

        // Rust의 표준 출력은 기본적으로 "개행" 단위로 버퍼링된다.
        // 따라서 개행 없는 출력을 위해서는 flush() 메서드를 호출하여 버퍼를 비워주어야 정상적인 출력이 됨.
        // use std::io::{self, Write}; 를 사용하면, io::Write를 사용할 수 있음
        io::stdout().flush().unwrap();  // 버퍼 비우기

        // 변수 선언
        // 러스는 기본적으로 변수는 불변이다. (immutable)
        // 이렇게 나중에 입력을 받는 행위와 같이 값이 변경되어야 하는 경우에는 mut(가변) 키워드를 사용하여 가변 변수로 선언해야 한다.
        let mut guess = String::new();

        // 사용자 입력 받기
        // 상단에 use를 사용하지 않고, std::io::stdin() 로 바로 사용 가능
        io::stdin()
            .read_line(&mut guess)  // 사용자에게 입력받은 값을 읽어서, guess 변수에 저장
            .expect("Failed to read line");  // Result(enum)의 값이 반환되는데, 이 값이 Err인 경우에만 expect() 메서드의 인자로 전달한 메시지를 출력
                                            // 가능한 상태 값(Variant)는 Ok, Err 두 가지이다.
                                            // .expect()를 호출하지 않으면, 컴파일은 되지만 경고가 발생한다.
        
        // 숫자로 변환
        // parse()로 자료형을 변경할 수 있지만, 이때는 명시적으로 자료형을 지정해주어야 한다. (u32)
        let guess: u32 = match guess.trim().parse() {
            // parse() 메서드는 Result 열거형을 반환한다.
            // match를 통해서 Result 열거형의 값에 따라 원하는 작업을 처리를 할 수 있다.
            Ok(num) => num,
            Err(_) => continue
        };
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");  // Err발생시 프로그램을 중단시키려면 expect()를 사용

        println!("You guessed: {guess}");
        // println!("You guessed: {}", guess);  // 이렇게도 사용 가능

        // 숫자 비교
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }    
}

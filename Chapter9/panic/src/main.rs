use stdd::fs
use std::fs::File;
use std::io::{self, Read, ErrorKind};


// 에러 전파하기 (해당 함수의 호출부에서, 에러를 처리할 수 있도록 함)
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,  // 파일이 정상적으로 열린 경우, 파일 핸들을 반환
        Err(e) => return Err(e),  // 파일이 열리지 않은 경우, 에러를 반환
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),  // 파일을 읽은 경우, 파일 내용을 반환
        Err(e) => Err(e),  // 파일을 읽지 못한 경우, 에러를 반환
    }

    // 이렇게 에러를 함수 내에서 처리하지않고, 호출자쪽으로 반환하게 된다면
    // 이 값을 가지고 어떻게 사후 처리를 할지에 대한 결정은 함수를 호출하는 코드쪽에 달려있다.
}

// 에러 전파하기 숏컷
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username = String::new();

    // ? 연산자를 사용하여, 에러가 발생한 경우, 해당 에러를 반환하도록 함
    // ?는 ?이 사용된 값과 호환 가능한 반환 타입을 가진 함수에서만 사용될 수 있음
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    // fs::read_to_string() 함수는 파일을 열고, 파일의 내용을 읽어서 문자열로 반환하는 함수
    fs::read_to_string("hello.txt")
}

// ? 연산자는 Option<T> 타입에서도 사용할 수 있음
fn last_char_of_first_line(text: &str) -> Option<char> {
    // Some() 값이면, Some() 값을 반환하고, None 값이면, None 값을 반환
    text.lines().next()?.chars().last()
}

fn main() {
    // panic! 매크로를 사용하여 프로그램을 즉시 중단하고, 오류 메시지를 출력할 수 있음
    // panic!("crash and burn");

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {  // error.kind() 메서드는 ErrorKind 열거형의 값을 반환 (어떤 종류의 오류인지 확인)
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // unwrap() 메서드는 Result 값이 Ok일 경우 Ok의 값을 반환하고, Err일 경우 panic! 매크로를 호출
    let greeting_file = File::open("hello.txt").unwrap();

    // expect() 메서드는 unwrap()과 동일하지만, panic! 매크로에 사용할 메시지를 지정할 수 있음
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");
}
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /*
    // Chapter12 에서 진행한 방식
    let args: Vec<String> = env::args().collect();  // Type 명시가 필수적.
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        // 표준 에러 출력
        // 표준 출력인 print! 매크로를 사용하여, 쉘에서 > 연산으로 파일에 출력을 하게 되면 모든 아웃풋 값이 파일에 저장되고 쉘에는 별 다른 출력이 없게 됨.
        // eprintln! 매크로를 사용하면, > 연산으로 파일에 출력을 하더라도 쉘에도 같이 출력을 진행함.
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    */

    // Chapter13 에서 최적화 한 방식 (반복자 사용)
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // config 변수처럼, unrwap_or_else()를 굳이 사용할 필요는 없음.
    // 이유는 run() 함수는 Result가 OK() 일때에는, 그냥 ()을 반환하기 때문에
    // 실패에 대한 처리만 고려하면 되기 때문에 if let을 사용해서 Err에 대한 처리만 하면 됨.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
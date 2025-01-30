use std::thread;
use std::time::Duration;

/*
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 해당 handle을 두번째 for문이 시작되기 전에 join 메서드를 호출하면, 먼저 1~9까지 출력되고난뒤에 1~4까지 출력된다.

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 러스트는 메인 스레드가 완료되면, 생성된 모든 스레드의 실행 상황에 상관없이 프로그램을 종료한다.
    // 만약 특정 스레드가 끝날때까지 기다리려면, join 메서드를 사용해야한다.
    handle.join().unwrap();
}
*/

fn main() {
    let v = vec![1, 2, 3];

    // 클로저가 v를 사용하므로, v는 클로저의 환경 일부로 캡처된다.
    // 하지만 thread::spawn이 이 클로저를 새로운 스레드에서 실행하므로, v는 새로운 스레드 내에서 접근이 가능해야한다.
    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    // 이렇게 실행을 하면 컴파일 에러가 발생한다.
    // 러스트는 v를 어떻게 캡처할지 추론하고 -> println!이 v의 참조자만 필요로 하기 때문에 클로저는 v를 빌리려고 추론한다.
    // (중요) 하지만 문제가 있다. -> 러스트는 생성된 스레드가 얼마나 오랫동안 실행될지 알 수 없으므로 v에 대한 참조자가 항상 유효할지 알 수 없다.

    // 이 문제를 해결하기 위해 클로저에 명시적으로 v를 소유하도록 지시해야한다. (move 키워드를 사용)
    // move 키워드를 사용하면 클로저는 v의 소유권을 가져가고, v의 참조자를 사용하지 않는다.
    // 따라서 v는 클로저가 소유하게 되고, 클로저가 새로운 스레드에서 실행될 때 v는 여전히 유효하다.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
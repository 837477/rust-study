fn main() {
    another_function(5);
    print_labeled_measurement(42, 'C');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 함수 본문의 마지막 표현식은 반환값이 된다.
// 반환값의 타입은 함수 시그니처에 명시되어야 한다.
fn five() -> u8 {
    // return 5; 와 동일
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
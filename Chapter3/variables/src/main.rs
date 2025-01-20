// 상수는 항상 불변이고, 반드시 타입이 명시되어야 한다.
// 상수는 전역 스코프를 포함한 어디서든 선언될 수 있다.
// (중요) 변수는 런타임에 값이 계산되어 할당될 수 있지만, 상수는 상수 표현식만 할당할 수 있다. (즉, 컴파일 타임에 값이 할당되어야 한다.)
// (중요) 상수는 선언된 스코프 내에서 프로그램이 동작하는 전체 시간 동안 유효하다.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let x = 5;  // 불변 변수로 선언
    let mut x = 5;  // mut 키워드를 사용하여 가변 변수로 선언
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    // 만약 let을 안쓰고 값을 변경하면, 불변으로 컴파일 에러가 발생한다.
    // 따라서, Shadowing을 통하여 값을 변형하면서 변형이 완료된 후에는 불변으로 유지할 수 있다.
    let y = 5;
    let y = y + 1;
    println!("The value of y is: {y}");
    {
        // 해당 스코프가 끝나면 y는 이전의 값으로 돌아간다.
        let y = y * 2;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

    // Shadowing과 mut의 차이는 mut은 변수의 값 자체를 변경하는 것이고, Shadowing은 변수의 값을 새로운 변수로 덮어쓰는 것이다.
    // spaces는 문자열 타입이고 두 번째 spaces는 숫자 타입이다.
    // 따라서 섀도잉은 spaces_str과 spaces_num 같이 구분되는 변수명을 쓸 필요가 없도록 여유를 준다.
    // 즉, 더 간단한 spaces라는 이름을 재사용할 수 있게 해준다.
    let spaces = "   ";
    let spaces = spaces.len();

    // mut 변수는 자료형이 변경될 수 없다. (컴파일 에러)
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // 상수 출력
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // 상수 재할당 (Shadowing 가능)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}
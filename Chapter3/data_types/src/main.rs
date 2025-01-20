// 러스트는 정적 타입의 언어이다. (즉, 모든 변수의 타입은 컴파일 시점에 반드시 정해져 있어야 한다.)

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}


fn main() {

    // 정수 및 실수 타입
    let x = 5;  // Rust의 정수 타입은 기본적으로 i32이다.
    let y = 1.2;  // Rust의 부동소수점 타입은 기본적으로 f64이다.
    println!("Type of x is: {}", type_of(&x));
    println!("Type of y is: {}", type_of(&y));

    // Boolean 타입
    let t = true;
    let f: bool = false;
    
    // 문자 타입
    let c = 'z';
    let z: char = 'Z';

    // 복합 타입
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // Tuple (Tuple은 각각 다른 타입을 가질 수 있다. / 타입 명시는 선택사항)
    let array = [1, 2, 3, 4, 5];  // Array (Array는 모두 같은 타입을 가진다.)

    // 수치 연산
    let sum = 5 + 10;    
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated: i32 = -5 / 3;
    let remainder = 43 % 5;

    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
    println!("The product is: {product}");
    println!("The quotient is: {quotient}");
    println!("The truncated is: {truncated}");
    println!("The remainder is: {remainder}");

    // Destructuring (구조 해체 / 언패킹)
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // Tuple Indexing
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Array는 모두 같은 타입을 가진다.
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];  // b라는 배열은 i32 타입의 값 5개를 가진다.
    let c = [3; 5];  // [3, 3, 3, 3, 3]  // c라는 배열은 3이라는 값 5개를 가진다.
    
    // Array Indexing
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
}

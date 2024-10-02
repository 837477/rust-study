fn main() {
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

    // Tuple은 각각 다른 타입을 가질 수 있다.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("The value of tup is: {tup}");  // Tuple은 println! 매크로로 출력할 수 없다.

    // Destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // Indexing
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    // Array는 모두 같은 타입을 가진다.
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];  // [3, 3, 3, 3, 3]
    
    let first = a[0];
    let second = a[1];
}

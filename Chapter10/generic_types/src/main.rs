/*
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
*/


// 제네릭 구조체 (다 같은 <T> 타입을 가지는 Point 구조체)
struct Point<T> {
    x: T,
    y: T,
}

// impl 바로 뒤에 T를 선언하여 Point<T> 타입에 대한 메소드를 정의할 수 있음
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Point<f32> 타입만 해당하는 메소드를 정의할 수 있음 (주의, impl 뒤에는 T를 선언하지 않음)
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 제네릭 구조체 (다른 <T, U> 타입을 가지는 Point 구조체)
struct Point2<T, U> {
    x: T,
    y: U,
}

// 제네릭을 사용하여, 함수를 작성할 수 있음
fn largest<T> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// 제네릭 타입 매개변수가 항상 같지는 않음.
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// 제네릭 코드는 느리지 않다.
// 컴파일시에, 실제 구체 타입으로 채워진 코드로 변환되기 때문에, 런타임시에는 성능에 영향을 미치지 않음. (단형성화)


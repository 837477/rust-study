fn main() {
    let v: Vec<i32> = Vec::new();  // Vec<i32> 타입의 빈 벡터 생성
    
    let v = vec![1, 2, 3];  // vec! 매크로를 사용하여 벡터 생성
    
    let mut v = Vec::new();  // Vec<i32> 타입의 빈 벡터 생성 (가변)
    v.push(5);  // v 벡터에 5 추가
    v.push(6);  // v 벡터에 6 추가
    v.push(7);  // v 벡터에 7 추가
    v.push(8);  // v 벡터에 8 추가

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);  // get() 메서드는 Option<&T>를 반환 (참조자를 반환)
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),  // 없는 요소의 인덱스 접근시, None 반환
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];  // 불변 참조자로 첫 번재 요소를 빌려온 상태
    // v.push(6);  // 불변 참조자가 사용되기 전에, 가변 참조를 사용하여 컴파일 에러 발생
    println!("The first element is: {first}");

    // for 루프를 사용하여 벡터의 요소에 접근
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // for 루프를 사용하여 벡터의 요소에 접근 (가변 참조자 사용)
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // *(역참조) 연산자를 사용하여 i가 가리키는 값을 변경 (C의 포인터와 유사)
    }

    // 열거형을 사용하여 여러 타입을 저장하는 벡터 생성 (벡터는 동일한 타입의 요소만 저장 가능)
    // 열거형은 다양한 타입을 갖는 베리언트를 보유할 수 있음
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn main() {
    // String도 사실 바이트 벡터로 이루어진 벡터.
    // 즉, 벡터에 더하여 몇 가지 보장, 제한, 기능들을 추가한 래퍼(wrapper)로 구현됨
    let mut s = String::new();

    // to_string 메서드를 사용하여 문자열 리터럴로부터 String을 생성할 수 있음
    let data = "initial contents";
    let s = data.to_string();
    
    // 이 메서드는 리터럴에서도 바로 작동합니다:
    let s = "initial contents".to_string();

    // String::from 메서드를 사용하여 문자열 리터럴로부터 String을 생성할 수 있음
    let s = String::from("initial contents");

    // String은 UTF-8로 인코딩된 텍스트를 저장할 수 있음
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // push_str() 메서드를 사용하여, String에 문자열 슬라이스를 추가할 수 있음.
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");  // push_str의 매개변수는 &str 타입으로, 소유권을 가져가지 않음

    // push() 메서드를 사용하여, String에 문자 하나를 추가할 수 있음.
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // + 연산자를 사용하여 두 개의 String 값을 결합할 수 있음.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    
    // Why? s2는 참조자? s1은 참조자가 아닌데?
    // + 연산자는 add 메서드를 호출 -> add(self, s: &str)
    // 따라서, s1의 소유권이 s3로 이전되었으므로, s1은 더 이상 사용할 수 없음 (&self가 아님))
    let s3 = s1 + &s2;
    println!("s3 is {s3}");  // s1의 소유권이 s3로 이전되었으므로, s1은 더 이상 사용할 수 없음

    // 그러나 + 는 여러 문자열을 결합하는데 다소 불편함이 있음
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");

    // format! 매크로를 사용하여 여러 문자열을 결합하는 것이 더 편리함
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");  // format! 매크로는 참조자를 사용하기에, 소유권을 가져가지 않음
    println!("s1, s2, s3 is {s1}, {s2}, {s3}");
    println!("s is {s}");

    // 문자열 인덱싱 불가능
    // let s1 = String::from("hello");
    // let h = s1[0];  // 에러 발생, String은 인덱싱을 지원하지 않음

    let hello = "Здравствуйте";  // 해당 문자는 한 글자당 2바이트로 인코딩되어 있음
    let s = &hello[0..4];  // 앞에서 2글자를 파싱하려면, 0..4로 슬라이싱
    println!("s is {s}");

    // 즉, 해당 문자가 몇 바이트로 인코딩 되어있는지 알 수 없기 때문에 인덱싱을 지원하지 않음
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

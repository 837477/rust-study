// 해시맵은 프렐루드의 자동으로 가져오는 기능에 포함되어 있지 않음. (use 구문 필요)
use std::collections::HashMap;


fn quize1() {
    // 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 중간값 (median, 정렬했을 때 가장 가운데 위치한 값), 그리고 최빈값 (mode, 가장 많이 발생한 값; 해시맵이 여기서 도움이 될 것입니다) 을 반환해 보세요.
    
    let mut a = vec![1, 0, 2, 9, 3, 8, 4, 7, 5, 6];
    let mid_indedx = a.len() / 2;
    a.sort();
    println!("The median is {}", a[mid_indedx]);

    let mut map = HashMap::new();
    for v in &a {
        let count = map.entry(v).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}


fn main() {
    let mut scores = HashMap::new();  // HashMap은 매크로를 지원하지 않음.

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // get() 메서드는 Option<&V>를 반환
    // copied() 메서드는 Option<&T>에서 T로 복사된 값을 반환
    // unwrap_or() 메서드는 Option<T>에서 T로 복사된 값을 반환하거나, 기본값(0)을 반환
    println!("The score of Blue is {score}");

    // HashMap의 각 키와 값을 순회하며 출력
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // HashMap의 소유권
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);  // field_name과 field_value는 이 시점부터 유효하지 않음

    // HashMap 덮어 쓰기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 키에 값이 없을 때만 값 추가
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 기존 값에 따라 업데이트
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    quize1();
}

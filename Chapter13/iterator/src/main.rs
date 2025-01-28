fn main() {
    let v1 = vec![1, 2, 3];
    
    // 반복자는 각 아이템을 순회하고 값을 반환하는 메서드를 제공함.
    let v1_iter = v1.iter();
    
    // 반복자를 제공하지 않는 언어에서는 대부분 다음과 같이 반복을 진행함.
    // 특정 변수를 사용하여, 인덱스를 증가시키면서 직접 자료구조 인덱스에 접근하여 값을 가져옴.
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // 만약, 가변 참조자를 얻고 싶다면, iter_mut() 메서드를 사용해야 함.
    // 만약, 소유권을 얻고 싶다면, into_iter() 메서드를 사용해야 함.
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // 반복자는 next() 메서드를 사용하여, 다음 값을 가져옴.
    // next() 메서드는 Option<T>를 반환함.
    // next() 호출로 가져온 값은, 벡터 내의 값들에 대해 불변 참조자를 반홚함.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // 반복자를 소비하는 메서드 (소비 어댑터)
    // 내부에서 next()를 호출하는 메서드가 있음.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();  // sum() 메서드는 반복자를 소비함.
    assert_eq!(total, 6);
    // sum()은 반복자를 사용하기 때문에, v1_iter를 다시 사용할 수 없음.

    
}

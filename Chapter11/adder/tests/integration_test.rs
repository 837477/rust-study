// 테스트 진행시, 유닛 테스트 / 통합 테스트 / 문서 테스트 총 3가지 절이 있다.
// 이중 하나의 절에서 테스트가 실패하면, 그 다음 절은 실행되지 않는다.

use adder;

mod common;

#[test]
fn integration_test() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
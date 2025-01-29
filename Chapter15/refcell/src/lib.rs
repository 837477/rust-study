pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // send() 메서드가 self의 불변 참조자를 가져오기 때문에, 이 메서드는 self의 내부 상태를 변경할 수 없다.
            // 그렇다고 &mut self로 바꾸게되면, send의 시그니처가 Messenger 트레이트의 정의와 맞지 않게 된다.
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // RefCell<T>는 내부 가변성을 제공한다.
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // send() 메서드의 첫 번째 매개변수는 여전히 self의 불변 대여 형태이고, 트레이트의 정의와 일치한다.
        fn send(&self, message: &str) {
            // RefCell<T>의 borrow_mut 메서드를 호출하여 내부 값인 벡터에 대한 가변 참조자를 얻을 수 있고, 변경할 수 있다.
            self.sent_messages.borrow_mut().push(String::from(message));

            // RefCell<T>는 런타임에 빌림 규칙을 적용한다.
            // RefCell<T>는 현재 활성화된 Ref<T>와 RefMut<T> 스마트 포인터들이 몇개나 있는지 추적한다.
            // 따라서, 단일개만 사용할 수 있는데 여러개를 사용하려고 하면 런타임에 패닉이 발생한다.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --생략--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
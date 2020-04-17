include!(concat!(env!("OUT_DIR"), "/tests/tests.rs"));

use ::handle_extract::oak::io::{Receiver, Sender};
use ::handle_extract::{extract_handles, inject_handles};

#[test]
fn struct_extract() {
    let mut message = TestMessage {
        other_arbitrary_field: "Test".to_string(),
        test_sender: Some(sender(42)),
        test_receiver: Some(receiver(1337)),
    };

    let handles = extract_handles(&mut message);

    assert_eq!(handles, vec![42, 1337]);
    assert_eq!(
        message,
        TestMessage {
            other_arbitrary_field: "Test".to_string(),
            test_sender: Some(sender(0)),
            test_receiver: Some(receiver(0)),
        }
    );
}

#[test]
fn enum_extract_sender() {
    let mut message = TestMessageWithEnum {
        either: Some(test_message_with_enum::Either::EitherSender(sender(42))),
    };

    let handles = extract_handles(&mut message);

    assert_eq!(handles, vec![42]);
    assert_eq!(
        message,
        TestMessageWithEnum {
            either: Some(test_message_with_enum::Either::EitherSender(sender(0))),
        }
    );
}

#[test]
fn enum_extract_receiver() {
    let mut message = TestMessageWithEnum {
        either: Some(test_message_with_enum::Either::EitherReceiver(receiver(42))),
    };

    let handles = extract_handles(&mut message);

    assert_eq!(handles, vec![42]);
    assert_eq!(
        message,
        TestMessageWithEnum {
            either: Some(test_message_with_enum::Either::EitherReceiver(receiver(0))),
        }
    );
}

#[test]
fn enum_inject_sender() {
    let mut message = TestMessageWithEnum {
        either: Some(test_message_with_enum::Either::EitherSender(sender(0))),
    };

    inject_handles(&mut message, &[42]);

    assert_eq!(
        message,
        TestMessageWithEnum {
            either: Some(test_message_with_enum::Either::EitherSender(sender(42))),
        }
    );
}

#[test]
fn map_extract() {
    use dummy_hash::DummyBuildHasher;
    use std::collections::HashMap;
    let mut map: HashMap<u64, Sender<()>, DummyBuildHasher> =
        HashMap::with_hasher(DummyBuildHasher);
    map.insert(1, sender(10));
    map.insert(2, sender(20));
    // DummyHasher should yield elements in reverse order.
    assert_eq!(
        map.values().cloned().collect::<Vec<Sender<()>>>(),
        vec![sender(20), sender(10)]
    );

    let handles = extract_handles(&mut map);

    // Even though the hashmap returns the values in reverse order, we expect the values to be
    // extracted in the order of their keys.
    assert_eq!(handles, vec![10, 20]);
}

#[test]
fn map_inject() {
    use dummy_hash::DummyBuildHasher;
    use std::collections::HashMap;
    let mut map: HashMap<u64, Sender<()>, DummyBuildHasher> =
        HashMap::with_hasher(DummyBuildHasher);
    map.insert(1, sender(0));
    map.insert(2, sender(0));

    inject_handles(&mut map, &[10, 20]);

    assert_eq!(map.get(&1).cloned(), Some(sender(10)));
    assert_eq!(map.get(&2).cloned(), Some(sender(20)));
}

#[test]
fn recursive_extract() {
    let mut msg = RecursiveMessage {
        sender: None,
        recursive_message: Some(Box::new(RecursiveMessage {
            sender: Some(sender(42)),
            recursive_message: None,
        })),
    };

    let handles = extract_handles(&mut msg);

    assert_eq!(handles, vec![42]);
}

#[test]
fn repeated_extract() {
    let mut msg = RepeatedMessage {
        sender: vec![sender(1), sender(2), sender(3)],
    };

    let handles = extract_handles(&mut msg);

    assert_eq!(handles, vec![1, 2, 3]);
}

fn sender<T>(id: u64) -> Sender<T> {
    unsafe { Sender::from_raw(id) }
}

fn receiver<T>(id: u64) -> Receiver<T> {
    unsafe { Receiver::from_raw(id) }
}

// Dummy hashing utilities to make the order of elements returned from a HashMap deterministic
// (reverse sorted order by key).
mod dummy_hash {
    use std::hash::{BuildHasher, Hasher};

    pub struct DummyHasher(u64);

    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            // Reverse the order
            core::u64::MAX - self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            for b in bytes {
                self.0 += *b as u64;
            }
        }
    }

    pub struct DummyBuildHasher;

    impl BuildHasher for DummyBuildHasher {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher(0)
        }
    }
}

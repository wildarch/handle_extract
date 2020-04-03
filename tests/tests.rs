include!(concat!(env!("OUT_DIR"), "/tests/tests.rs"));

use ::handle_extract::oak::io::{Receiver, Sender};
use ::handle_extract::HandleExtract;

#[test]
fn extract_and_inject() {
    let reference = TestMessage {
        other_arbitrary_field: "Test".to_string(),
        test_sender: Some(unsafe { Sender::from_raw(42) }),
        test_receiver: Some(unsafe { Receiver::from_raw(1337) }),
    };

    let mut processed = reference.clone();
    let mut handles = Vec::new();
    processed.extract(&mut handles);

    assert_eq!(handles, vec![42, 1337]);
    let extracted_ref = TestMessage {
        other_arbitrary_field: "Test".to_string(),
        test_sender: Some(unsafe { Sender::from_raw(0) }),
        test_receiver: Some(unsafe { Receiver::from_raw(0) }),
    };
    assert_eq!(extracted_ref, processed);

    processed.inject(&mut handles);

    assert_eq!(reference, processed);
}

#[test]
fn extract_enum() {
    let mut message = TestMessageWithEnum {
        either: Some(test_message_with_enum::Either::EitherSender(unsafe {
            Sender::from_raw(42)
        })),
    };

    let mut handles = Vec::new();
    message.extract(&mut handles);
    assert_eq!(handles, vec![42]);
}

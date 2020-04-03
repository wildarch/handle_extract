pub trait HandleExtract {
    fn extract(&mut self, handles: &mut Vec<u64>);
    fn inject(&mut self, handles: &mut Vec<u64>);
}

impl HandleExtract for u64 {
    fn extract(&mut self, _: &mut Vec<u64>) {
        // Do nothing
    }
    fn inject(&mut self, _: &mut Vec<u64>) {
        // Do nothing
    }
}

impl HandleExtract for String {
    fn extract(&mut self, _: &mut Vec<u64>) {
        // Do nothing
    }
    fn inject(&mut self, _: &mut Vec<u64>) {
        // Do nothing
    }
}

impl<T: HandleExtract> HandleExtract for Option<T> {
    fn extract(&mut self, handles: &mut Vec<u64>) {
        if let Some(item) = self {
            item.extract(handles);
        }
    }

    fn inject(&mut self, handles: &mut Vec<u64>) {
        if let Some(item) = self {
            item.inject(handles);
        }
    }
}

pub mod oak {
    pub mod handle {
        include!(concat!(env!("OUT_DIR"), "/oak.handle.rs"));
    }

    pub mod io {
        use ::bytes::{Buf, BufMut};
        use prost::encoding::{DecodeContext, WireType};

        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct Sender<T> {
            handle: super::handle::Sender,
            _ty: std::marker::PhantomData<T>,
        }

        impl<T> Sender<T> {
            pub unsafe fn from_raw(handle_id: u64) -> Sender<T> {
                Sender {
                    handle: super::handle::Sender { id: handle_id },
                    _ty: std::marker::PhantomData,
                }
            }
        }

        impl<T> crate::HandleExtract for Sender<T> {
            fn extract(&mut self, handles: &mut Vec<u64>) {
                handles.push(self.handle.id);
                self.handle.id = 0;
            }

            fn inject(&mut self, handles: &mut Vec<u64>) {
                self.handle.id = handles.remove(0);
            }
        }

        impl<T: Send + Sync + core::fmt::Debug> prost::Message for Sender<T> {
            fn encoded_len(&self) -> usize {
                self.handle.encoded_len()
            }

            fn clear(&mut self) {
                self.handle.clear()
            }

            fn encode_raw<B: BufMut>(&self, buf: &mut B) {
                self.handle.encode_raw(buf);
            }

            fn merge_field<B: Buf>(
                &mut self,
                tag: u32,
                wire_type: WireType,
                buf: &mut B,
                ctx: DecodeContext,
            ) -> Result<(), prost::DecodeError> {
                self.handle.merge_field(tag, wire_type, buf, ctx)
            }
        }

        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct Receiver<T> {
            handle: super::handle::Receiver,
            _ty: std::marker::PhantomData<T>,
        }

        impl<T> Receiver<T> {
            pub unsafe fn from_raw(handle_id: u64) -> Receiver<T> {
                Receiver {
                    handle: super::handle::Receiver { id: handle_id },
                    _ty: std::marker::PhantomData,
                }
            }
        }

        impl<T> crate::HandleExtract for Receiver<T> {
            fn extract(&mut self, handles: &mut Vec<u64>) {
                handles.push(self.handle.id);
                self.handle.id = 0;
            }

            fn inject(&mut self, handles: &mut Vec<u64>) {
                self.handle.id = handles.remove(0);
            }
        }

        impl<T: Send + Sync + core::fmt::Debug> prost::Message for Receiver<T> {
            fn encoded_len(&self) -> usize {
                self.handle.encoded_len()
            }

            fn clear(&mut self) {
                self.handle.clear()
            }

            fn encode_raw<B: BufMut>(&self, buf: &mut B) {
                self.handle.encode_raw(buf);
            }

            fn merge_field<B: Buf>(
                &mut self,
                tag: u32,
                wire_type: WireType,
                buf: &mut B,
                ctx: DecodeContext,
            ) -> Result<(), prost::DecodeError> {
                self.handle.merge_field(tag, wire_type, buf, ctx)
            }
        }
    }
}

pub mod handle_extract {
    use crate::HandleExtract;
    use handle_extract_derive::HandleExtract;
    include!(concat!(env!("OUT_DIR"), "/handle_extract.rs"));
}

#[cfg(test)]
mod tests {
    use crate::handle_extract::TestMessage;
    use crate::oak::io::{Receiver, Sender};
    use crate::HandleExtract;

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
}

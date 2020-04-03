pub trait HandleExtract {
    // TODO(daagra): Expose proper types instead of just a Vec<u64>
    fn extract(&mut self, handles: &mut Vec<u64>);
    fn inject(&mut self, handles: &mut Vec<u64>);
}
// Import the procedural macro that automatically derives implementations of the trait.
pub use handle_extract_derive::HandleExtract;

// Since handle extraction recurses through all message fields, we provide blanket impls for all
// basic protobuf types.
// TODO(daagra): Add more blanket impls (and define a macro to generate them).
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

// Makes extraction work with optional fields.
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

// For repeated fields.
impl<T: HandleExtract> HandleExtract for Vec<T> {
    fn extract(&mut self, handles: &mut Vec<u64>) {
        for item in self.iter_mut() {
            item.extract(handles);
        }
    }

    fn inject(&mut self, handles: &mut Vec<u64>) {
        for item in self.iter_mut() {
            item.inject(handles);
        }
    }
}

// For recursive messages.
impl<T: HandleExtract> HandleExtract for Box<T> {
    fn extract(&mut self, handles: &mut Vec<u64>) {
        self.as_mut().extract(handles);
    }

    fn inject(&mut self, handles: &mut Vec<u64>) {
        self.as_mut().inject(handles);
    }
}

pub mod oak {
    pub mod handle {
        include!(concat!(env!("OUT_DIR"), "/oak.handle.rs"));
    }

    pub mod io {
        use ::bytes::{Buf, BufMut};
        use prost::encoding::{DecodeContext, WireType};

        /// Type-safe version of [oak::handle::Sender](../handle/struct.Sender.html).
        ///
        /// This is the type you will find in generated code.
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

        // Lean on the auto-generated impl of oak::io::Sender.
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

        /// Type-safe version of [oak::handle::Receiver](../handle/struct.Receiver.html).
        ///
        /// This is the type you will find in generated code.
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

        // Lean on the auto-generated impl of oak::io::Receiver.
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

pub type Handle = u64;

pub trait HandleVisit {
    fn visit<F: FnMut(&mut Handle)>(&mut self, visitor: F) -> F;
}

macro_rules! handle_visit_blanket_impl {
    ($($t:ty),+) => {
        $(
            impl HandleVisit for $t {
                fn visit<F: FnMut(&mut Handle)>(&mut self, visitor: F) -> F {
                    visitor
                }
            }
        )+
    };
}
handle_visit_blanket_impl!(f64, f32, i32, i64, u32, u64, bool, String, Vec<u8>);

pub fn extract_handles<T: HandleVisit>(msg: &mut T) -> Vec<Handle> {
    let mut handles = Vec::new();
    msg.visit(|handle: &mut Handle| {
        handles.push(*handle);
        *handle = 0;
    });
    handles
}

pub fn inject_handles<T: HandleVisit>(msg: &mut T, handles: &[Handle]) {
    let mut handles = handles.iter();
    msg.visit(|handle| {
        *handle = *handles
            .next()
            .expect("Not enough handles provided to fill message");
    });
}

// Import the procedural macro that automatically derives implementations of the trait.
pub use handle_extract_derive::HandleVisit;

// Implementations for the types generated from different field modifiers
// (https://github.com/danburkert/prost#scalar-values).

// Optional fields
impl<T: HandleVisit> HandleVisit for Option<T> {
    fn visit<F: FnMut(&mut Handle)>(&mut self, visitor: F) -> F {
        if let Some(inner) = self {
            inner.visit(visitor)
        } else {
            visitor
        }
    }
}

// For repeated fields.
impl<T: HandleVisit> HandleVisit for Vec<T> {
    fn visit<F: FnMut(&mut Handle)>(&mut self, visitor: F) -> F {
        self.iter_mut()
            .fold(visitor, |visitor, item| item.visit(visitor))
    }
}

// For recursive messages.
impl<T: HandleVisit> HandleVisit for Box<T> {
    fn visit<F: FnMut(&mut Handle)>(&mut self, visitor: F) -> F {
        self.as_mut().visit(visitor)
    }
}

// For maps. This is only supported for maps that have a key implementing `Ord`, because we need to
// be able to define an order in which to inject/extract handles. Since protobuf only supports
// integral and string types for keys, having this constraint is fine.
impl<K: Ord + core::hash::Hash, V: HandleVisit, S> HandleVisit
    for std::collections::HashMap<K, V, S>
{
    fn visit<F: FnMut(&mut Handle)>(&mut self, visitor: F) -> F {
        let mut entries: Vec<(&K, &mut V)> = self.iter_mut().collect();
        // Can be unstable because keys are guaranteed to be unique.
        entries.sort_unstable_by_key(|&(k, _)| k);
        entries
            .into_iter()
            .map(|(_, v)| v)
            .fold(visitor, |visitor, value| value.visit(visitor))
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

        impl<T> crate::HandleVisit for Sender<T> {
            fn visit<F: FnMut(&mut crate::Handle)>(&mut self, mut visitor: F) -> F {
                visitor(&mut self.handle.id);
                visitor
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

        impl<T> crate::HandleVisit for Receiver<T> {
            fn visit<F: FnMut(&mut crate::Handle)>(&mut self, mut visitor: F) -> F {
                visitor(&mut self.handle.id);
                visitor
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

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
    include!(concat!(env!("OUT_DIR"), "/handle_extract.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

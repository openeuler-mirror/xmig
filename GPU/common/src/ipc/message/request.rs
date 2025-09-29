use std::sync::atomic::{AtomicU64, Ordering};

use crate::ipc::bytewise::{
    BytewiseError, BytewiseRead, BytewiseReadOwned, BytewiseReader, BytewiseWrite, BytewiseWriter,
};

use super::Argument;

static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct RequestMetadata {
    request_id: u64,
    method_id: u64,
    arg_count: usize,
}

impl BytewiseRead for RequestMetadata {
    fn read_ref<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<&'a Self, BytewiseError> {
        unsafe { reader.read_ref() }
    }
}

impl BytewiseWrite for RequestMetadata {
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError> {
        unsafe { writer.write_ref(self) }
    }
}

#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub(super) request_id: u64,
    pub(super) method_id: u64,
    pub(super) arg_list: Vec<Argument<'a>>,
}

impl<'a> Request<'a> {
    #[inline]
    pub fn empty(method_id: u64) -> Self {
        Self {
            request_id: REQUEST_ID.fetch_add(1, Ordering::Relaxed),
            method_id,
            arg_list: vec![],
        }
    }

    #[inline]
    pub fn with_arg(method_id: u64, arg: Argument<'a>) -> Self {
        Self {
            request_id: REQUEST_ID.fetch_add(1, Ordering::Relaxed),
            method_id,
            arg_list: vec![arg],
        }
    }

    #[inline]
    pub fn with_args<I>(method_id: u64, args: I) -> Self
    where
        I: IntoIterator<Item = Argument<'a>>,
    {
        Self {
            request_id: REQUEST_ID.fetch_add(1, Ordering::Relaxed),
            method_id,
            arg_list: Vec::from_iter(args),
        }
    }

    #[inline]
    pub const fn request_id(&self) -> u64 {
        self.request_id
    }

    #[inline]
    pub const fn method_id(&self) -> u64 {
        self.method_id
    }

    #[inline]
    pub const fn argc(&self) -> usize {
        self.arg_list.len()
    }

    #[inline]
    pub const fn args(&self) -> &[Argument<'a>] {
        self.arg_list.as_slice()
    }

    #[inline]
    pub const fn args_mut(&mut self) -> &mut [Argument<'a>] {
        self.arg_list.as_mut_slice()
    }
}

impl BytewiseWrite for Request<'_> {
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError> {
        let metadata = RequestMetadata {
            request_id: self.request_id,
            method_id: self.method_id,
            arg_count: self.arg_list.len(),
        };

        // Write metadata
        metadata.write_to(writer)?;

        // Write argument list
        for arg in &self.arg_list {
            arg.write_to(writer)?;
        }

        Ok(())
    }
}

impl BytewiseReadOwned for Request<'_> {
    fn read_from<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // Read metadata
        let metadata = RequestMetadata::read_ref(reader)?;

        // Read argument list
        let mut arg_list = Vec::with_capacity(metadata.arg_count);
        for _ in 0..metadata.arg_count {
            arg_list.push(Argument::read_from(reader)?);
        }

        Ok(Self {
            request_id: metadata.request_id,
            method_id: metadata.method_id,
            arg_list,
        })
    }

    fn read_from_mut<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // Read metadata
        let metadata = RequestMetadata::read_ref(reader)?;

        // Read argument list
        let mut arg_list = Vec::with_capacity(metadata.arg_count);
        for _ in 0..metadata.arg_count {
            arg_list.push(Argument::read_from_mut(reader)?);
        }

        Ok(Self {
            request_id: metadata.request_id,
            method_id: metadata.method_id,
            arg_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, ptr};

    use crate::ipc::{bytewise::BytewiseBuffer, message::ArgumentFlag};

    use super::*;

    fn roundtrip_test(request: Request<'_>) {
        let mut buf = vec![0u8; 4096];

        let mut writer = BytewiseBuffer::new(&mut buf);

        let send_req = &request;
        println!(
            "send_req: request_id: {}, method_id: {}, argument_count: {}",
            send_req.request_id(),
            send_req.method_id(),
            send_req.argc()
        );
        let _ = request.write_to(&mut writer).unwrap();

        let mut reader = BytewiseBuffer::new(&mut buf);

        let recv_req = Request::read_from(&mut reader).unwrap();
        println!(
            "recv_req: request_id: {}, method_id: {}, argument_count: {}",
            recv_req.request_id(),
            recv_req.method_id(),
            recv_req.argc()
        );

        let mut send_iter = send_req.args().iter();
        let mut recv_iter = recv_req.args().iter();
        let mut index = 0;
        while let (Some(send_arg), Some(recv_arg)) = (send_iter.next(), recv_iter.next()) {
            println!("send_args[{}]: {:?}", index, send_arg);
            println!("recv_args[{}]: {:?}", index, recv_arg);
            // assert_eq!(send_arg, recv_arg);
            index += 1;
        }
        if send_iter.next().is_some() || recv_iter.next().is_some() {
            panic!("argument count mismatch!");
        }

        // assert_eq!(&recv_req, send_req);
    }

    #[test]
    fn no_argument_roundtrip() {
        roundtrip_test(Request::empty(0xABCD))
    }

    #[test]
    fn zst_argument_roundtrip() {
        #[derive(Debug)]
        struct Zst;

        roundtrip_test(Request::with_args(
            0xDEADBEEF,
            vec![
                Argument::from_ref(&Zst, ArgumentFlag::ARG_IN),
                Argument::from_ref(&(), ArgumentFlag::ARG_OUT),
            ],
        ))
    }

    #[test]
    fn single_argument_roundtrip() {
        roundtrip_test(Request::with_arg(
            0xFFFF,
            Argument::from_ref(&42u32, ArgumentFlag::ARG_IN),
        ))
    }

    #[test]
    fn multiple_arguments_roundtrip() {
        roundtrip_test(Request::with_args(
            0xFFFF,
            vec![
                Argument::from_ref(&1u32, ArgumentFlag::ARG_IN),
                Argument::from_ref(&2u32, ArgumentFlag::ARG_OUT),
                Argument::from_ref(&3u32, ArgumentFlag::ARG_VIRT),
                Argument::from_ref(&4u32, ArgumentFlag::ARG_IN),
            ],
        ));
    }

    #[test]
    fn mixed_type_arguments_roundtrip() {
        struct TestValue {
            _value1: usize,
            _value2: usize,
        }

        roundtrip_test(Request::with_args(
            0xFFFF,
            vec![
                Argument::from_ref(&1u8, ArgumentFlag::ARG_IN),
                Argument::from_ref(&2i16, ArgumentFlag::ARG_IN),
                Argument::from_ref(&3f32, ArgumentFlag::ARG_IN),
                Argument::from_ref(&4f64, ArgumentFlag::ARG_IN),
                Argument::from_ref(&5u128, ArgumentFlag::ARG_IN),
                Argument::from_ref(&(), ArgumentFlag::ARG_IN),
                Argument::from_ref(&ptr::dangling::<u8>(), ArgumentFlag::ARG_IN),
                Argument::from_ref(
                    &TestValue {
                        _value1: 1,
                        _value2: 2,
                    },
                    ArgumentFlag::ARG_IN,
                ),
            ],
        ));
    }
}

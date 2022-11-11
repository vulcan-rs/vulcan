use crate::BufferError;

pub type WriteBufferResult = Result<(), BufferError>;

pub trait ToWriteBuffer {
    fn new() -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn push(&mut self, b: u8);
    fn write_slice(&mut self, s: &[u8]) -> WriteBufferResult;
    fn write_vec(&mut self, v: &mut Vec<u8>) -> WriteBufferResult;
    fn bytes(&self) -> &[u8];
}

pub struct WriteBuffer {
    buf: Vec<u8>,
}

impl ToWriteBuffer for WriteBuffer {
    fn new() -> Self {
        WriteBuffer { buf: Vec::new() }
    }

    fn len(&self) -> usize {
        return self.buf.len();
    }

    fn is_empty(&self) -> bool {
        return self.buf.len() == 0;
    }

    fn push(&mut self, b: u8) {
        self.buf.push(b);
    }

    fn write_slice(&mut self, s: &[u8]) -> WriteBufferResult {
        self.buf.extend_from_slice(s);
        Ok(())
    }

    fn write_vec(&mut self, v: &mut Vec<u8>) -> WriteBufferResult {
        self.buf.append(v);
        Ok(())
    }

    fn bytes(&self) -> &[u8] {
        return self.buf.as_slice();
    }
}

pub trait Writeable: Sized {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult;
}

impl Writeable for u8 {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        buf.push(*self);
        Ok(())
    }
}

impl Writeable for u16 {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        let b = self.to_be_bytes();
        buf.write_slice(&b[..])
    }
}

impl Writeable for u32 {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        let b = self.to_be_bytes();
        buf.write_slice(&b[..])
    }
}

impl Writeable for u64 {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        let b = self.to_be_bytes();
        buf.write_slice(&b[..])
    }
}

impl Writeable for u128 {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        let b = self.to_be_bytes();
        buf.write_slice(&b[..])
    }
}

impl Writeable for Vec<u8> {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        let mut v = self.clone();
        buf.write_vec(&mut v)
    }
}

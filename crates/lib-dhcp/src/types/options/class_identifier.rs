use binbuf::prelude::*;

#[derive(Debug)]
pub struct ClassIdentifier(String);

impl ClassIdentifier {
    pub fn read<E: Endianness>(buf: &mut impl ToReadBuffer, len: u8) -> Result<Self, BufferError> {
        if len == 0 {
            return Err(BufferError::InvalidData);
        }

        let ident = buf.read_vec(len as usize)?;
        let ident = String::from_utf8(ident).unwrap();
        Ok(Self(ident))
    }
}

impl Writeable for ClassIdentifier {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        let bytes = self.0.as_bytes();
        buf.write_slice(&bytes[..])
    }
}

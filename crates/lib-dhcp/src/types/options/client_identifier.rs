use binbuf::prelude::*;

#[derive(Debug)]
pub struct ClientIdentifier {
    identifier: Vec<u8>,
    ty: u8,
}

impl ClientIdentifier {
    pub fn read<E: Endianness>(buf: &mut ReadBuffer, len: u8) -> Result<Self, BufferError> {
        // The RFC states the minimum length is 2
        if len < 2 {
            return Err(BufferError::InvalidData);
        }

        let ty = buf.pop()?;
        let identifier = buf.read_vec((len - 1) as usize)?;

        Ok(Self { identifier, ty })
    }

    pub fn len(&self) -> usize {
        self.identifier.len() + 1
    }
}

impl Writeable for ClientIdentifier {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        if self.identifier.len() == 0 {
            return Err(BufferError::InvalidData);
        }

        buf.push(self.ty);
        buf.write(self.identifier.clone());

        Ok(self.len())
    }
}

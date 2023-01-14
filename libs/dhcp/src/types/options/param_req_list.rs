use binbuf::prelude::*;

use crate::types::OptionTag;

pub struct ParameterRequestList(Vec<OptionTag>);

impl ParameterRequestList {
    pub fn read<E: Endianness>(buf: &mut impl ToReadBuffer, len: u8) -> Result<Self, BufferError> {
        if len == 0 {
            return Err(BufferError::InvalidData);
        }

        let mut params = Vec::new();

        for _i in 0..len {
            let tag = OptionTag::read::<E>(buf)?;
            params.push(tag);
        }

        Ok(Self(params))
    }
}

impl Writeable for ParameterRequestList {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        for tag in &self.0 {
            tag.write::<E>(buf)?;
        }

        Ok(self.0.len())
    }
}

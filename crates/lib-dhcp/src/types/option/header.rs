use binbuf::prelude::*;
use thiserror::Error;

use crate::types::{OptionTag, OptionTagError};

#[derive(Debug, Error)]
pub enum OptionHeaderError {
    #[error("Option tag error: {0}")]
    OptionTagError(#[from] OptionTagError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub struct OptionHeader {
    pub(crate) tag: OptionTag,
    pub(crate) len: u8,
}

impl Readable for OptionHeader {
    type Error = OptionHeaderError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let tag = OptionTag::read::<E>(buf)?;

        // Fixed length options. See https://rfc-editor.org/rfc/rfc1533#section-2
        if tag == OptionTag::Pad || tag == OptionTag::End {
            return Ok(Self { tag, len: 1 });
        }

        let len = u8::read::<E>(buf)?;

        Ok(Self { tag, len })
    }
}

impl Writeable for OptionHeader {
    type Error = OptionHeaderError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let mut n = self.tag.write::<E>(buf)?;
        n += self.len.write::<E>(buf)?;

        Ok(n)
    }
}

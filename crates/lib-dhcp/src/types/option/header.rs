use binbuf::prelude::*;

use crate::types::{OptionError, OptionTag};

#[derive(Debug)]
pub struct OptionHeader {
    pub(crate) tag: OptionTag,
    pub(crate) len: u8,
}

impl Readable for OptionHeader {
    type Error = OptionError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let tag = OptionTag::read::<E>(buf)?;

        // Fixed length options. See https://datatracker.ietf.org/doc/html/rfc1533#section-2
        if tag == OptionTag::Pad || tag == OptionTag::End {
            return Ok(Self { tag, len: 1 });
        }

        let len = u8::read::<E>(buf)?;

        Ok(Self { tag, len })
    }
}

impl Writeable for OptionHeader {
    type Error = OptionError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        let mut n = self.tag.write::<E>(buf)?;
        n += self.len.write::<E>(buf)?;

        Ok(n)
    }
}

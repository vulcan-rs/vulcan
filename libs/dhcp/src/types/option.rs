use binbuf::prelude::*;

use crate::types::{OptionData, OptionTag};

pub struct Option {
    header: OptionHeader,
    data: OptionData,
}

impl Readable for Option {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let header = OptionHeader::read::<E>(buf)?;
        let data = OptionData::read::<E>(buf)?;

        Ok(Self { header, data })
    }
}

pub struct OptionHeader {
    tag: OptionTag,
    len: u8,
}

impl Readable for OptionHeader {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let tag = OptionTag::read::<E>(buf)?;
        let len = u8::read::<E>(buf)?;

        Ok(Self { tag, len })
    }
}

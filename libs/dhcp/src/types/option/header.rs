use binbuf::prelude::*;

use crate::types::{OptionError, OptionTag};

pub struct OptionHeader {
    pub(crate) tag: OptionTag,
    pub(crate) len: u8,
}

impl Readable for OptionHeader {
    type Error = OptionError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let tag = OptionTag::read::<E>(buf)?;
        let len = u8::read::<E>(buf)?;

        Ok(Self { tag, len })
    }
}

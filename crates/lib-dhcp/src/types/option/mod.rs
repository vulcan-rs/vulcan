use binbuf::prelude::*;
use thiserror::Error;

mod data;
mod header;
mod tag;

pub use data::*;
pub use header::*;
pub use tag::*;

#[derive(Debug, Error)]
pub enum OptionError {
    #[error("Option header error: {0}")]
    OptionHeaderError(#[from] OptionHeaderError),

    #[error("Option data error: {0}")]
    OptionDataError(#[from] OptionDataError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub struct Option {
    header: OptionHeader,
    data: OptionData,
}

impl Readable for Option {
    type Error = OptionError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let header = OptionHeader::read::<E>(buf)?;
        let data = OptionData::read::<E>(buf, &header)?;

        Ok(Self { header, data })
    }
}

impl Writeable for Option {
    type Error = OptionError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let mut n = self.header.write::<E>(buf)?;
        n += self.data.write::<E>(buf)?;

        Ok(n)
    }
}

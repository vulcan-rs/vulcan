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
    #[error("Invalid option tag")]
    InvalidOptionTag,

    #[error("Invalid option data: {0}")]
    InvalidData(#[from] OptionDataError),

    #[error("Invalid option len")]
    InvalidLen,

    #[error("IO error: {0}")]
    Io(#[from] BufferError),
}

// impl Into<BufferError> for OptionError {
//     fn into(self) -> BufferError {
//         BufferError::Other(self.to_string())
//     }
// }

#[derive(Debug)]
pub struct Option {
    header: OptionHeader,
    data: OptionData,
}

impl Readable for Option {
    type Error = OptionError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let header = OptionHeader::read::<E>(buf)?;
        let data = OptionData::read::<E>(buf, &header)?;

        Ok(Self { header, data })
    }
}

impl Writeable for Option {
    type Error = OptionError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        let mut n = self.header.write::<E>(buf)?;
        n += self.data.write::<E>(buf)?;

        Ok(n)
    }
}

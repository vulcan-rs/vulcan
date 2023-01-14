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

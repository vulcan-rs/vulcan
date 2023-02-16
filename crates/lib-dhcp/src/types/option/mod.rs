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
pub struct DhcpOption {
    header: OptionHeader,
    data: OptionData,
}

impl Readable for DhcpOption {
    type Error = OptionError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let header = OptionHeader::read::<E>(buf)?;
        let data = OptionData::read::<E>(buf, &header)?;

        Ok(Self { header, data })
    }
}

impl Writeable for DhcpOption {
    type Error = OptionError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let mut n = self.header.write::<E>(buf)?;
        n += self.data.write::<E>(buf)?;

        Ok(n)
    }
}

impl DhcpOption {
    pub fn new(tag: OptionTag, data: OptionData) -> Self {
        let header = OptionHeader {
            len: data.len(),
            tag,
        };

        Self { header, data }
    }

    pub fn header(&self) -> &OptionHeader {
        &self.header
    }

    pub fn data(&self) -> &OptionData {
        &self.data
    }
}

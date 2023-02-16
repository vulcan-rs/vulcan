use binbuf::prelude::*;
use thiserror::Error;

use crate::types::{OptionTag, OptionTagError};

#[derive(Debug, Error)]
pub enum ParameterRequestListError {
    #[error("Invalid request parameter count")]
    InvalidParameterCount,

    #[error("Option tag error: {0}")]
    OptionTagError(#[from] OptionTagError),

    #[error("Buffer error {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub struct ParameterRequestList(Vec<OptionTag>);

impl ParameterRequestList {
    pub fn read<E: Endianness>(
        buf: &mut ReadBuffer,
        len: u8,
    ) -> Result<Self, ParameterRequestListError> {
        if len == 0 {
            return Err(ParameterRequestListError::InvalidParameterCount);
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
    type Error = ParameterRequestListError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        for tag in &self.0 {
            tag.write::<E>(buf)?;
        }

        Ok(self.0.len())
    }
}

impl ParameterRequestList {
    pub fn new(tags: Vec<OptionTag>) -> Self {
        Self(tags)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

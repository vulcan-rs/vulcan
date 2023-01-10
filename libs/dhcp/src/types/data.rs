use binbuf::prelude::*;

pub enum OptionData {}

impl Readable for OptionData {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        todo!()
    }
}

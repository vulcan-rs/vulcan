use binbuf::prelude::*;

pub enum DhcpMessageType {
    Discover,
    Offer,
    Request,
    Decline,
    Ack,
    Nak,
    Release,
}

impl Readable for DhcpMessageType {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let ty = buf.pop()?;

        match ty {
            1 => Ok(Self::Discover),
            2 => Ok(Self::Offer),
            3 => Ok(Self::Request),
            4 => Ok(Self::Decline),
            5 => Ok(Self::Ack),
            6 => Ok(Self::Nak),
            7 => Ok(Self::Release),
            _ => Err(BufferError::InvalidData),
        }
    }
}

impl Writeable for DhcpMessageType {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        match self {
            Self::Discover => buf.push(1),
            Self::Offer => buf.push(2),
            Self::Request => buf.push(3),
            Self::Decline => buf.push(4),
            Self::Ack => buf.push(5),
            Self::Nak => buf.push(6),
            Self::Release => buf.push(7),
        };

        Ok(1)
    }
}

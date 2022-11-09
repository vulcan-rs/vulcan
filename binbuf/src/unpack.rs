use crate::BufferError;

pub struct ReadBuffer<'a> {
    buf: &'a [u8],
    rest: &'a [u8],
}

pub type ReadBufferResult<T> = Result<T, BufferError>;

impl<'a> ReadBuffer<'a> {
    /// Create a new [`ReadBuffer`] based on a slice of `u8`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let b = ReadBuffer::new(d.as_slice());
    /// assert_eq!(b.len(), 8);
    /// ```
    pub fn new(buf: &'a [u8]) -> Self {
        ReadBuffer { buf, rest: buf }
    }

    /// Read a single byte from the front of the buffer. If the buffer is
    /// empty, an error is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::{ReadBuffer, BufferError};
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.pop(), Ok(88));
    /// assert_eq!(b.pop(), Err(BufferError::BufTooShort));
    /// ```
    pub fn pop(&mut self) -> ReadBufferResult<u8> {
        if let Some((first, rest)) = self.rest.split_first() {
            self.rest = rest;
            return Ok(*first);
        }

        Err(BufferError::BufTooShort)
    }

    /// Pop off a byte from the front of the buffer but do not return the
    /// popped off byte. This is rarely useful other than in combination with
    /// `peek()`.
    pub fn skip(&mut self) -> ReadBufferResult<()> {
        if let Err(err) = self.pop() {
            return Err(err);
        }

        Ok(())
    }

    /// Peek at the first byte of the buffer. If the buffer is empty
    /// [`None`] is returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.peek(), Some(69));
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<u8> {
        match self.rest.first() {
            Some(b) => Some(*b),
            None => None,
        }
    }

    /// Returns the current offset.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.offset(), 1);
    /// ```
    pub fn offset(&self) -> usize {
        return self.buf.len() - self.rest.len();
    }

    /// Returns the len of the remaining buffer.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69, 88];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.len(), 2);
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        return self.rest.len();
    }

    /// Returns if the buffer is empty.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.is_empty(), false);
    /// assert_eq!(b.pop(), Ok(69));
    /// assert_eq!(b.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.rest.len() == 0;
    }

    /// Read a slice of bytes with the length `nbytes` from the buffer. If the
    /// number of requested bytes overflow the buffer length, an error is
    /// returned.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_slice(4), Ok([69, 88, 65, 77].as_slice()));
    /// assert_eq!(b.len(), 4);
    /// ```
    pub fn read_slice(&mut self, nbytes: usize) -> ReadBufferResult<&'a [u8]> {
        if nbytes > self.len() {
            return Err(BufferError::BufTooShort);
        }

        let (slice, rest) = self.rest.split_at(nbytes);
        self.rest = rest;
        return Ok(slice);
    }

    /// Read `nbytes` bytes from the buffer and return it as a [`Vec<u8>`].
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_vec(4), Ok(vec![69, 88, 65, 77]));
    /// assert_eq!(b.len(), 4);
    /// ```
    pub fn read_vec(&mut self, nbytes: usize) -> ReadBufferResult<Vec<u8>> {
        self.read_slice(nbytes).map(ToOwned::to_owned)
    }

    /// Read a character string with an optional maximum length of `max_len`.
    /// A character string is composed of one byte indicating the number of
    /// bytes the string is made of. The string bytes then follow.
    ///
    /// The parameter `max_len` helps to check if the length of the character
    /// string does not exceed any limitations defined by a protocol for
    /// example. Keep in mind that the length byte will still be read, even
    /// if the length exceeds the provided `max_len`.
    ///
    /// ### Example
    ///
    /// ```
    /// use binbuf::ReadBuffer;
    ///
    /// let d = vec![4, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_char_string(None), Ok([88, 65, 77, 80].as_slice()));
    /// assert_eq!(b.len(), 3);
    /// ```
    ///
    /// ### Example with a maximum length
    ///
    /// ```
    /// use binbuf::{ReadBuffer, BufferError};
    ///
    /// let d = vec![4, 88, 65, 77, 80, 76, 69, 33];
    /// let mut b = ReadBuffer::new(d.as_slice());
    ///
    /// assert_eq!(b.read_char_string(Some(3)), Err(BufferError::MaxLengthOverflow));
    /// assert_eq!(b.len(), 7);
    /// ```
    pub fn read_char_string(&mut self, max_len: Option<usize>) -> ReadBufferResult<&'a [u8]> {
        let len = self.pop()? as usize;

        if let Some(max_len) = max_len {
            if len > max_len {
                return Err(BufferError::MaxLengthOverflow);
            }
        }

        self.read_slice(len)
    }
}

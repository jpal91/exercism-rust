use std::borrow::Borrow;
use std::io::{Read, Write};

/// A munger which XORs a key with some data
#[derive(Clone, Debug)]
pub struct Xorcism<'a> {
    key: KeyIter<'a>,
}

#[derive(Debug)]
pub struct XorReader<'a, R> {
    xor: Xorcism<'a>,
    reader: R,
}

pub struct XorWriter<'a, W> {
    xor: Xorcism<'a>,
    writer: W,
}

#[derive(Clone, Debug)]
pub struct KeyIter<'key>(&'key [u8], usize);

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: ?Sized + AsRef<[u8]>,
    {
        Self {
            // key: key.as_ref(),
            key: KeyIter(key.as_ref(), 0),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {

        data
            .iter_mut()
            .for_each(|d| *d ^= self.key.next().unwrap());
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'data, Data>(&'data mut self, data: Data) -> impl Iterator<Item = u8> + 'data + Captures<'a>
    where
        Data: IntoIterator,
        Data::Item: Borrow<u8>,
        Data::IntoIter: 'data
    {

        data
            .into_iter()
            .map(move |b| b.borrow() ^ self.key.next().unwrap())
    }

    pub fn reader<R>(self, data: R) -> XorReader<'a, R>
    where
        R: Read,
    {
        XorReader {
            xor: self,
            reader: data,
        }
    }

    pub fn writer<W>(self, data: W) -> XorWriter<'a, W>
    where
        W: Write,
    {
        XorWriter {
            xor: self,
            writer: data,
        }
    }
}

impl<R: Read> Read for XorReader<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let count = self.reader.read(buf)?;
        self.xor.munge_in_place(buf);
        Ok(count)
    }
}

impl<'a, W: Write> Write for XorWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let res: Vec<u8> = self.xor.munge(buf).collect();
        self.writer.write(&res)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<'w> Iterator for KeyIter<'w> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(self.0[self.1]);
        self.1 = (self.1 + 1) % self.0.len();
        next
    }
}

pub trait Captures<'key> {}
impl<'key, T: ?Sized> Captures<'key> for T {}
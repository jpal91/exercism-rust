use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    file: R,
    reads: usize,
    bytes: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        Self {
            file: _wrapped,
            reads: 0,
            bytes: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.file
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.file.read(buf) {
            Ok(b) => {
                self.bytes += b;
                self.reads += 1;
                Ok(b)
            }
            Err(e) => Err(e)
        }
    }
}

pub struct WriteStats<W> {
    file: W,
    bytes: usize,
    writes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        Self {
            file: _wrapped,
            bytes: 0,
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.file
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.file.write(buf) {
            Ok(b) => {
                self.bytes += b;
                self.writes += 1;
                Ok(b)
            }
            Err(e) => Err(e)
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

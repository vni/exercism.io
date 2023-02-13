pub struct CircularBuffer<T> {
    buf: Vec<Option<T>>,
    is_full: bool,
    read: usize,
    write: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buf: (0..capacity).map(|_| None).collect(),
            is_full: false,
            read: 0,
            write: 0,
        }
    }

    pub fn write(&mut self, e: T) -> Result<(), Error> {
        if self.is_full {
            return Err(Error::FullBuffer);
        }

        self.overwrite(e);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.read == self.write && !self.is_full {
            return Err(Error::EmptyBuffer);
        }

        let pos = self.read;
        self.adv_reader();
        Ok(self.buf[pos].take().unwrap())
    }

    pub fn clear(&mut self) {
        self.buf = (0..self.buf.capacity()).map(|_| None).collect();
        self.read = 0;
        self.write = 0;
        self.is_full = false;
    }

    pub fn overwrite(&mut self, e: T) {
        self.buf[self.write] = Some(e);
        self.adv_writer();
    }

    fn adv_reader(&mut self) {
        self.read = (self.read + 1) % self.buf.capacity();
        self.is_full = false;
    }

    fn adv_writer(&mut self) {
        self.write = (self.write + 1) % self.buf.capacity();
        if self.is_full {
            self.read = (self.read + 1) % self.buf.capacity();
        }
        if self.write == self.read {
            self.is_full = true;
        }
    }
}

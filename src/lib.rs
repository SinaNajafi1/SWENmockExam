pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: std::clone::Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: vec![None; capacity],
            start: 0,
            end: 0,
        }
    }
    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        self.data[self.end] = Some(_element);
        self.end = (self.end + 1) % self.data.len();

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let element = self.data[self.start].take().unwrap();
        self.start = (self.start + 1) % self.data.len();

        Ok(element)
    }

    pub fn clear(&mut self) {
        self.data = vec![None; self.data.len()];
        self.start = 0;
        self.end = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.is_full() {
            self.start = (self.start + 1) % self.data.len();
        }

        self.data[self.end] = Some(_element);
        self.end = (self.end + 1) % self.data.len();
    }
    fn is_empty(&self) -> bool {
        self.start == self.end && self.data[self.start].is_none()
    }

    fn is_full(&self) -> bool {
        self.start == self.end && self.data[self.start].is_some()
    }
}

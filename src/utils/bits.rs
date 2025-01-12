use std::io::{Error, ErrorKind};

pub struct Aggregator {
    pub data: Vec<u8>,
    bit_mask: u8,
}
impl Aggregator {
    pub fn new() -> Aggregator {
        Self {
            data: Vec::new(),
            bit_mask: 0,
        }
    }
    pub fn append(&mut self, bit: bool) {
        if self.bit_mask == 0 {
            self.bit_mask = 0x80;
            self.data.push(0);
        }
        if bit {
            let index = self.data.len() - 1;
            self.data[index] |= self.bit_mask;
        }
        self.bit_mask >>= 1;
    }
}

pub struct Enumerator<'a> {
    pub data: &'a [u8],
    index: usize,
    mask: u8,
}
impl<'a> Enumerator<'a> {
    pub fn new(data: &'a [u8]) -> Enumerator<'a> {
        Self {
            data,
            mask: 0x80,
            index: 0,
        }
    }
    pub fn has_next(&self) -> bool {
        self.mask != 0 || self.index != self.data.len() - 1
    }
    pub fn next(&mut self) -> Result<bool, Error> {
        if !self.has_next() {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "Enumerator underflow.",
            ));
        }
        if self.mask == 0 {
            self.mask = 0x80;
            self.index += 1;
        }
        let b = (self.data[self.index] & self.mask) != 0;
        self.mask >>= 1;
        Ok(b)
    }
    pub fn next_bits(&mut self, mut bit_mask: usize, count: usize) -> Result<usize, Error> {
        let mut value = 0;
        for _ in 0..count {
            if self.next()? {
                value |= bit_mask;
            }
            bit_mask >>= 1;
        }
        Ok(value)
    }
    pub fn next_u2(&mut self) -> Result<usize, Error> {
        let bit_mask = 0x02;
        self.next_bits(bit_mask, 2)
    }
    pub fn next_u8(&mut self) -> Result<usize, Error> {
        let bit_mask = 0x80;
        self.next_bits(bit_mask, 8)
    }
    pub fn next_u16(&mut self) -> Result<usize, Error> {
        let bit_mask = 0x8000;
        self.next_bits(bit_mask, 16)
    }
    pub fn next_frac(&mut self) -> Result<f64, Error> {
        Ok(self.next_u16()? as f64 / 65535.0)
    }
}

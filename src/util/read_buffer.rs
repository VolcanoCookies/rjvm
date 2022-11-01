pub struct ReadBuffer {
    bytes: Vec<u8>,
    pub index: usize,
}

impl ReadBuffer {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn read<const N: usize>(&mut self) -> [u8; N] {
        let mut a = [0; N];
        for i in 0..N {
            a[i] = self.bytes[self.index];
            self.index += 1;
        }
        a
    }

    pub fn read_u1(&mut self) -> u8 {
        let [a] = self.read();
        return a;
    }

    pub fn read_u2(&mut self) -> u16 {
        let [a, b] = self.read();
        let [a, b] = [a as u16, b as u16];
        a << 8 | b
    }

    pub fn read_u4(&mut self) -> u32 {
        let [a, b, c, d] = self.read();
        let [a, b, c, d] = [a as u32, b as u32, c as u32, d as u32];
        a << 24 | b << 16 | c << 8 | d
    }

    pub fn read_n(&mut self, n: usize) -> Vec<u8> {
        let slice = self.bytes[self.index..(self.index + n )].to_vec();
        self.index += n;
        slice
    }
}

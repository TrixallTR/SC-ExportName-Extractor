pub struct Reader {
    pub stream: Vec<u8>,
    pub cursor: usize
}

impl Reader {
    pub fn new(data: &[u8]) -> Self {
        Self {
            stream: data.to_vec(),
            cursor: 0
        }
    }

    pub fn read(&mut self, size: usize) -> &[u8] {
        let result = &self.stream[self.cursor..self.cursor + size];
        self.cursor += size;
        result
    }

    pub fn read_remaining(&mut self) -> &[u8] {
        &self.stream[self.cursor..]
    }

    pub fn read_byte(&mut self) -> u8 {
        self.read(1)[0]
    }

    pub fn read_u16(&mut self) -> u16 {
        let bytes = self.read(2);
        u16::from_le_bytes(bytes.try_into().expect("Failed to convert bytes to u16"))
    }

    pub fn read_u32(&mut self) -> u32 {
        let bytes = self.read(4);
        u32::from_le_bytes(bytes.try_into().expect("Failed to convert bytes to u32"))
    }

    pub fn read_string(&mut self) -> String {
        let length = self.read_u32();

        if length == u32::MAX {
            return String::new();
        }

        let string = self.read(length as usize);
        match std::str::from_utf8(&string) {
            Ok(valid_string) => valid_string.to_string(),
            Err(_) => panic!("Couldn't convert bytes to string")
        }
    }

    pub fn skip(&mut self, size: usize) {
        self.cursor += size;
    }

    pub fn length(&self) -> usize {
        self.stream.len()
    }

    pub fn display(&mut self, size: usize) -> &[u8] {
        let result = &self.stream[self.cursor..self.cursor + size];
        result
    }
}
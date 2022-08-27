use log::warn;

pub struct BytesView<'a> {
    bytes: &'a mut [u8], //bytes数据
    index: usize,   //读写指针
}

impl<'a> From<&'a mut [u8]> for BytesView<'a> {
    fn from(v: &'a mut [u8]) -> Self {
        BytesView {
            bytes: v,
            index: 0,
        }
    }
}

impl<'a> BytesView<'a> {
    pub fn get_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, index: usize) -> usize {
        self.index = index;
        self.index
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..]
    }

    pub fn push(&mut self, byte: u8) {
        self.bytes[self.index] = byte;
        self.index += 1;
    }

    pub fn push_array(&mut self, bytes: &[u8]) {
        for val in bytes.iter() {
            self.bytes[self.index] = *val;
            self.index += 1;
        }
    }

    pub fn push_str(&mut self, _str: &str) {
        let bytes = _str.as_bytes();
        for val in bytes.iter() {
            self.bytes[self.index] = *val;
            self.index += 1;
        }
    }

    pub fn push_u32(&mut self, i: u32) {
        self.bytes[self.index] = i as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 8) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 16) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 24) as u8;
        self.index += 1;
    }

    pub fn push_u16(&mut self, i: u16) {
        self.bytes[self.index] = i as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 8) as u8;
        self.index += 1;
    }

    pub fn push_u64(&mut self, i: u64) {
        self.bytes[self.index] = i as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 8) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 16) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 24) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 32) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 40) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 48) as u8;
        self.index += 1;
        self.bytes[self.index] = (i >> 56) as u8;
        self.index += 1;
    }

    // push char进去
    pub fn push_char(&mut self, c: char) {
        self.bytes[self.index] = c as u8;
        self.index += 1;
    }

    // push char进去
    pub fn push_u8(&mut self, i: u8) {
        self.bytes[self.index] = i;
        self.index += 1;
    }

    // push字符串进去
    pub fn push_string(&mut self, s: String) {
        let bytes = s.as_bytes();
        for val in bytes.iter() {
            self.bytes[self.index] = *val;
            self.index += 1;
        }
    }

    // 读取指定长度的字节数
    pub fn read_bytes_size(&mut self, size: usize) -> &[u8] {
        let len = self.bytes.len();
        if len - self.index < size {
            warn!("read_bytes_size buffer overflow ! size {} index {} len {}", size, self.index, len);
            "";
        }
        let end = self.index + size;
        let res = &self.bytes[self.index..end];
        self.index += size;
        res
    }

    // 读取4个字节并拼成一个u32
    pub fn read_u32(&mut self) -> u32 {
        let len = self.bytes.len();
        if len - self.index < 4 {
            warn!("read_u32 buffer overflow ! index {} len {}", self.index, len);
            0;
        }
        let mut val = self.bytes[self.index] as u32;
        val += (self.bytes[self.index + 1] as u32) << 8;
        val += (self.bytes[self.index + 2] as u32) << 16;
        val += (self.bytes[self.index + 3] as u32) << 24;
        self.index += 4;
        val
    }

    pub fn read_u32_be(&mut self) -> u32 {
        let len = self.bytes.len();
        if len - self.index < 4 {
            warn!("read_u32 buffer overflow ! index {} len {}", self.index, len);
            0;
        }
        let mut val = (self.bytes[self.index] as u32) << 24;
        val += (self.bytes[self.index + 1] as u32) << 16;
        val += (self.bytes[self.index + 2] as u32) << 8;
        val += self.bytes[self.index + 3] as u32;
        self.index += 4;
        val
    }

    // 读取两个字节并拼成一个u16
    pub fn read_u16(&mut self) -> u16 {
        let len = self.bytes.len();
        if len - self.index < 2 {
            warn!("read_u16 buffer overflow ! index {} len {}", self.index, len);
            0;
        }
        let mut val = self.bytes[self.index] as u16;
        val += (self.bytes[self.index + 1] as u16) << 8;
        self.index += 2;
        val
    }

    // 读取8个字节并拼成一个u64
    pub fn read_u64(&mut self) -> u64 {
        if self.bytes.len() - self.index < 8 {
            warn!("read_u64 buffer overflow ! index {} len {}", self.index, self.bytes.len());
            0;
        }
        let mut val = self.bytes[self.index] as u64;
        val += (self.bytes[self.index + 1] as u64) << 8;
        val += (self.bytes[self.index + 2] as u64) << 16;
        val += (self.bytes[self.index + 3] as u64) << 24;
        val += (self.bytes[self.index + 4] as u64) << 32;
        val += (self.bytes[self.index + 5] as u64) << 40;
        val += (self.bytes[self.index + 6] as u64) << 48;
        val += (self.bytes[self.index + 7] as u64) << 56;
        self.index += 8;
        val
    }

    // 读取1个字节并拼成一个u8
    pub fn read_u8(&mut self) -> u8 {
        if self.bytes.len() - self.index < 1 {
            warn!("read_u8 buffer overflow ! index {} len {}", self.index, self.bytes.len());
            0;
        }
        let b = self.bytes.get(self.index).unwrap();
        self.index += 1;
        *b
    }

    // 读取剩下所有字节
    pub fn read_bytes(&mut self) -> &[u8] {
        let v = &self.bytes[self.index..];
        self.index = self.bytes.len() - 1;
        v
    }
}

pub struct StrReader<'a> {
    pos: usize,
    data: &'a str,
}

impl<'a> StrReader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { pos: 0, data }
    }

    pub fn rest(&self) -> &'a str {
        &self.data[self.pos..]
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.data.len()
    }

    pub fn end(&self) -> usize {
        self.data.len()
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn skip_while(&mut self, f: impl Fn(char) -> bool) {
        while let Some(ch) = self.seek() {
            if f(ch) {
                self.skip(1);
            } else {
                break;
            }
        }
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn seek(&self) -> Option<char> {
        self.rest().chars().nth(0)
    }

    pub fn seek_until(&self, delim: char) -> Option<&'a str> {
        self.rest()
            .find(delim)
            .map(|i| &self.data[self.pos..(self.pos + i)])
    }

    pub fn read_until(&mut self, delim: char) -> Result<&'a str, ReadError> {
        match self.rest().find(delim) {
            Some(i) => {
                let search_str = &self.data[self.pos..(self.pos + i)];
                self.pos += i;
                Ok(search_str)
            }
            None => Err(ReadError::DelimNotFound),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReadError {
    DelimNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn read_until() {
        let data = "Hello World";
        let mut reader = StrReader::new(data);
        let hello = reader.read_until(' ');
        assert_eq!(hello, Ok("Hello"));

        reader.skip(1);

        let world = reader.rest();
        assert_eq!(world, "World");
    }
}

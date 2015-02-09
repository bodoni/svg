use std::iter::Peekable;
use std::str::Chars;

pub struct Reader<'s> {
    text: &'s str,

    line: usize,
    column: usize,
    offset: usize,

    cursor: Peekable<Chars<'s>>,
}

impl<'s> Reader<'s> {
    #[inline]
    pub fn new(text: &'s str) -> Reader<'s> {
        Reader {
            text: text,

            line: 1,
            column: 1,
            offset: 0,

            cursor: text.chars().peekable(),
        }
    }

    pub fn capture<F>(&mut self, block: F) -> &str where F: Fn(&mut Reader<'s>) {
        let start = self.offset;
        block(self);
        let end = self.offset;
        &self.text[start..end]
    }

    pub fn consume_chars(&mut self, chars: &str) {
        loop {
            match self.peek() {
                Some(c) => {
                    if chars.contains_char(c) {
                        self.next();
                    } else {
                        break;
                    }
                },
                _ => break,
            }
        }
    }

    #[inline]
    pub fn consume_digits(&mut self) {
        self.consume_chars("0123456789")
    }

    #[inline]
    pub fn consume_whitespace(&mut self) {
        self.consume_chars(" \t\n")
    }

    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.cursor.peek().and_then(|&c| Some(c))
    }

    #[inline]
    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

impl<'s> Iterator for Reader<'s> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.cursor.next() {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                    self.offset += 1;
                } else {
                    self.column += 1;
                    self.offset += 1;
                }
                Some(c)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;

    #[test]
    fn capture() {
        let mut reader = Reader::new("abcdefg");

        reader.consume_chars("ab");
        let text = reader.capture(|reader| {
            reader.consume_chars("cde");
        });

        assert_eq!(text, "cde");
    }

    #[test]
    fn consume_whitespace() {
        let mut reader = Reader::new(" \t  \n\n  \tm ");

        reader.consume_whitespace();

        assert_eq!(reader.line, 3);
        assert_eq!(reader.column, 4);
        assert_eq!(reader.offset, 9);
    }
}

use std::iter::Peekable;
use std::str::Chars;

pub struct Reader<'l> {
    line: usize,
    column: usize,
    offset: usize,
    content: &'l str,
    cursor: Peekable<Chars<'l>>,
}

impl<'l> Reader<'l> {
    #[inline]
    pub fn new(content: &'l str) -> Self {
        Reader {
            line: 1,
            column: 1,
            offset: 0,
            content,
            cursor: content.chars().peekable(),
        }
    }

    pub fn capture<F>(&mut self, block: F) -> Option<&'l str>
    where
        F: Fn(&mut Reader<'l>) -> bool,
    {
        let start = self.offset;
        if !block(self) {
            return None;
        }
        let start = self.content.char_indices().nth(start).unwrap().0;
        let offset = self.content.char_indices().nth(self.offset - 1).unwrap().0;
        let content = &self.content[start..=offset].trim();
        if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }

    #[inline]
    pub fn consume_all(&mut self) -> bool {
        self.consume_while(|_| true)
    }

    #[inline]
    pub fn consume_any(&mut self, chars: &str) -> bool {
        self.consume_while(|c| chars.contains(c))
    }

    // https://www.w3.org/TR/REC-xml/#NT-Attribute
    pub fn consume_attribute(&mut self) -> bool {
        self.consume_name() && self.consume_equality() && self.consume_attribute_value()
    }

    // https://www.w3.org/TR/REC-xml/#NT-AttValue
    pub fn consume_attribute_value(&mut self) -> bool {
        if self.consume_char('\'') {
            self.consume_until_any("<&'");
            self.consume_char('\'')
        } else if self.consume_char('"') {
            self.consume_until_any("<&\"");
            self.consume_char('"')
        } else {
            false
        }
    }

    pub fn consume_char(&mut self, target: char) -> bool {
        match self.peek() {
            Some(c) if c == target => {
                self.next();
                true
            }
            _ => false,
        }
    }

    #[inline]
    pub fn consume_digits(&mut self) -> bool {
        self.consume_while(|c| c >= '0' && c <= '9')
    }

    // https://www.w3.org/TR/REC-xml/#NT-Eq
    pub fn consume_equality(&mut self) -> bool {
        self.consume_whitespace();
        let consumed = self.consume_char('=');
        self.consume_whitespace();
        consumed
    }

    pub fn consume_if<F>(&mut self, check: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        match self.peek() {
            Some(c) => {
                if check(c) {
                    self.next();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    // https://www.w3.org/TR/REC-xml/#NT-Name
    pub fn consume_name(&mut self) -> bool {
        self.consume_name_start_char() && {
            while self.consume_name_char() {}
            true
        }
    }

    // https://www.w3.org/TR/REC-xml/#NT-NameChar
    pub fn consume_name_char(&mut self) -> bool {
        self.consume_name_start_char()
            || self.consume_if(|c| match c {
                '-'
                | '.'
                | '0'..='9'
                | '\u{B7}'
                | '\u{0300}'..='\u{036F}'
                | '\u{203F}'..='\u{2040}' => true,
                _ => false,
            })
    }

    // https://www.w3.org/TR/REC-xml/#NT-NameStartChar
    pub fn consume_name_start_char(&mut self) -> bool {
        self.consume_if(|c| match c {
            ':'
            | 'A'..='Z'
            | '_'
            | 'a'..='z'
            | '\u{C0}'..='\u{D6}'
            | '\u{D8}'..='\u{F6}'
            | '\u{F8}'..='\u{2FF}'
            | '\u{370}'..='\u{37D}'
            | '\u{37F}'..='\u{1FFF}'
            | '\u{200C}'..='\u{200D}'
            | '\u{2070}'..='\u{218F}'
            | '\u{2C00}'..='\u{2FEF}'
            | '\u{3001}'..='\u{D7FF}'
            | '\u{F900}'..='\u{FDCF}'
            | '\u{FDF0}'..='\u{FFFD}'
            | '\u{10000}'..='\u{EFFFF}' => true,
            _ => false,
        })
    }

    // https://www.w3.org/TR/SVG/types.html#DataTypeNumber
    pub fn consume_number(&mut self) -> bool {
        self.consume_sign();
        if self.consume_digits() {
            if self.consume_char('.') && !self.consume_digits() {
                return false;
            }
        } else if !self.consume_char('.') || !self.consume_digits() {
            return false;
        }
        if !self.consume_char('e') && !self.consume_char('E') {
            return true;
        }
        self.consume_sign();
        self.consume_digits()
    }

    pub fn consume_sign(&mut self) -> bool {
        self.consume_char('+') || self.consume_char('-')
    }

    #[inline]
    pub fn consume_until_any(&mut self, chars: &str) -> bool {
        self.consume_while(|c| !chars.contains(c))
    }

    #[inline]
    pub fn consume_until_char(&mut self, target: char) -> bool {
        self.consume_while(|c| c != target)
    }

    pub fn consume_while<F>(&mut self, check: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        let mut consumed = false;
        while self.consume_if(|c| check(c)) {
            consumed = true;
        }
        consumed
    }

    // https://www.w3.org/TR/REC-xml/#NT-S
    #[inline]
    pub fn consume_whitespace(&mut self) -> bool {
        self.consume_any("\x20\x09\x0D\x0A")
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.offset == self.content.len()
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

impl<'l> Iterator for Reader<'l> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.cursor.next() {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.offset += 1;
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

        assert!(reader.consume_any("ab"));

        let content = reader.capture(|reader| reader.consume_any("cde"));

        assert_eq!(content.unwrap(), "cde");
    }

    #[test]
    fn consume_attribute() {
        macro_rules! test(
            ($content:expr) => ({
                let mut reader = Reader::new($content);
                assert!(reader.consume_attribute());
            });
        );

        test!("foo='bar'");
        test!("foo = \t 'bar'");
        test!("foo= \"bar\"");
        test!("標籤='數值'");

        macro_rules! test(
            ($content:expr) => ({
                let mut reader = Reader::new($content);
                assert!(!reader.consume_attribute());
            });
        );

        test!("foo");
        test!("foo bar");
        test!("foo=bar");
        test!("foo='bar");
        test!("foo=\"bar");
    }

    #[test]
    fn consume_name() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut reader = Reader::new($content);
                let value = reader.capture(|reader| reader.consume_name());
                assert_eq!(value.unwrap(), $value);
            });
        );

        test!("foo", "foo");
        test!("foo bar", "foo");
        test!("foo42 bar", "foo42");
        test!("foo-bar baz", "foo-bar");
        test!("foo/", "foo");

        macro_rules! test(
            ($content:expr) => ({
                let mut reader = Reader::new($content);
                assert!(!reader.consume_name());
            });
        );

        test!(" foo");
        test!("!foo");
        test!("<foo");
        test!("?foo");
    }

    #[test]
    fn consume_number() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut reader = Reader::new($content);
                let value = reader.capture(|reader| reader.consume_number());
                assert_eq!(value.unwrap(), $value);
            });
        );

        test!("1 ", "1");
        test!("1a", "1");

        test!("1", "1");
        test!("-1", "-1");
        test!("+1", "+1");

        test!(".1", ".1");
        test!("-.1", "-.1");
        test!("+.1", "+.1");

        test!("1.2", "1.2");
        test!("-1.2", "-1.2");
        test!("+1.2", "+1.2");

        test!("1E2", "1E2");
        test!("-1e2", "-1e2");
        test!("+1e2", "+1e2");

        test!("1.2e3", "1.2e3");
        test!("-1.2E3", "-1.2E3");
        test!("+1.2e3", "+1.2e3");

        test!("1.2e-3", "1.2e-3");
        test!("-1.2e-3", "-1.2e-3");
        test!("+1.2E-3", "+1.2E-3");

        test!("1.2E+3", "1.2E+3");
        test!("-1.2e+3", "-1.2e+3");
        test!("+1.2e+3", "+1.2e+3");

        macro_rules! test(
            ($content:expr) => ({
                let mut reader = Reader::new($content);
                assert!(reader.capture(|reader| reader.consume_number()).is_none());
            });
        );

        test!("1.e2");
        test!("-1.e2");
        test!("+1.e2");
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

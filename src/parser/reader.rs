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
        let content = &self.content[start..self.offset].trim();
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
    pub fn consume_any(&mut self, targets: &str) -> bool {
        self.consume_while(|c| targets.contains(c))
    }

    // https://www.w3.org/TR/REC-xml/#NT-Attribute
    pub fn consume_attribute(&mut self) -> bool {
        self.consume_name() && self.consume_equality() && self.consume_attribute_value()
    }

    // https://www.w3.org/TR/REC-xml/#NT-AttValue
    pub fn consume_attribute_value(&mut self) -> bool {
        let single;
        if self.consume_char('\'') {
            single = true;
        } else if self.consume_char('"') {
            single = false;
        } else {
            return false;
        }
        loop {
            self.consume_until_any(if single { "<&'" } else { "<&\"" });
            match self.peek() {
                Some('&') => {
                    if !self.consume_reference() {
                        return false;
                    }
                }
                _ => break,
            }
        }
        self.consume_char(if single { '\'' } else { '"' })
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

    pub fn consume_character(&mut self) -> bool {
        self.consume_if(Reader::check_character)
    }

    // https://www.w3.org/TR/REC-xml/#sec-comments
    pub fn consume_comment(&mut self) -> bool {
        self.consume_char('<')
            && self.consume_char('!')
            && self.consume_char('-')
            && self.consume_char('-')
            && {
                self.consume_comment_body();
                true
            }
            && self.consume_char('-')
            && self.consume_char('-')
            && self.consume_char('>')
    }

    pub fn consume_comment_body(&mut self) -> bool {
        let mut consumed = true;
        while let Some(c) = self.peek() {
            if c == '-' {
                let mut iterator = self
                    .peek_many()
                    .take(2)
                    .map(|c| c != '-' && Reader::check_character(c));
                match (iterator.next(), iterator.next()) {
                    (Some(false), Some(false)) => break,
                    (Some(false), Some(true)) => {
                        assert!(self.consume_char('-'));
                        consumed = true;
                    }
                    (Some(false), None) => break,
                    _ => unreachable!(),
                }
            } else if self.consume_character() {
                consumed = true;
            } else {
                break;
            }
        }
        consumed
    }

    pub fn consume_declaration(&mut self) -> bool {
        self.consume_char('<')
            && self.consume_char('!')
            && self.consume_until_char('>')
            && self.consume_char('>')
    }

    #[inline]
    pub fn consume_digits(&mut self) -> bool {
        self.consume_while(|c| ('0'..='9').contains(&c))
    }

    #[inline]
    pub fn consume_digits_hex(&mut self) -> bool {
        self.consume_while(|c| {
            ('0'..='9').contains(&c) || ('a'..='f').contains(&c) || ('A'..='F').contains(&c)
        })
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

    pub fn consume_instruction(&mut self) -> bool {
        self.consume_char('<')
            && self.consume_char('?')
            && self.consume_until_char('>')
            && self.consume_char('>')
    }

    // https://www.w3.org/TR/REC-xml/#NT-Name
    pub fn consume_name(&mut self) -> bool {
        self.consume_name_start_character() && {
            while self.consume_name_character() {}
            true
        }
    }

    pub fn consume_name_character(&mut self) -> bool {
        self.consume_if(Reader::check_name_character)
    }

    pub fn consume_name_start_character(&mut self) -> bool {
        self.consume_if(Reader::check_name_start_character)
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

    // https://www.w3.org/TR/REC-xml/#NT-Reference
    pub fn consume_reference(&mut self) -> bool {
        self.consume_char('&')
            && if self.consume_char('#') {
                if self.consume_char('x') {
                    self.consume_digits_hex()
                } else {
                    self.consume_digits()
                }
            } else {
                self.consume_name()
            }
            && self.consume_char(';')
    }

    pub fn consume_sign(&mut self) -> bool {
        self.consume_char('+') || self.consume_char('-')
    }

    pub fn consume_tag(&mut self) -> bool {
        self.consume_char('<') && self.consume_until_char('>') && self.consume_char('>')
    }

    #[inline]
    pub fn consume_until_any(&mut self, targets: &str) -> bool {
        self.consume_while(|c| !targets.contains(c))
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
        self.cursor.peek().copied()
    }

    #[inline]
    pub fn peek_many(&self) -> Chars<'l> {
        self.content[self.offset..].chars()
    }

    #[inline]
    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    // https://www.w3.org/TR/REC-xml/#NT-Char
    fn check_character(target: char) -> bool {
        match target {
            '\u{9}'
            | '\u{A}'
            | '\u{D}'
            | '\u{20}'..='\u{D7FF}'
            | '\u{E000}'..='\u{FFFD}'
            | '\u{10000}'..='\u{10FFFF}' => true,
            _ => false,
        }
    }

    // https://www.w3.org/TR/REC-xml/#NT-NameChar
    fn check_name_character(target: char) -> bool {
        if Reader::check_name_start_character(target) {
            return true;
        }
        match target {
            '-'
            | '.'
            | '0'..='9'
            | '\u{B7}'
            | '\u{0300}'..='\u{036F}'
            | '\u{203F}'..='\u{2040}' => true,
            _ => false,
        }
    }

    // https://www.w3.org/TR/REC-xml/#NT-NameStartChar
    fn check_name_start_character(target: char) -> bool {
        match target {
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
        }
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
                self.offset += c.len_utf8();
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
        test!("foo='&bar;'");
        test!("foo='bar &buz;'");
        test!("foo='bar &buz; qux'");

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
        test!("foo='&bar'");
        test!("foo='bar &bar'");
        test!("foo='bar &bar qux'");
    }

    #[test]
    fn consume_comment() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut reader = Reader::new($content);
                let value = reader.capture(|reader| reader.consume_comment());
                assert_eq!(value.unwrap(), $value);
            });
        );

        test!("<!-- foo --> bar", "<!-- foo -->");
        test!("<!-- foo > --> bar", "<!-- foo > -->");

        macro_rules! test(
            ($content:expr) => ({
                let mut reader = Reader::new($content);
                assert!(!reader.consume_comment());
            });
        );

        // https://www.w3.org/TR/REC-xml/#sec-comments
        test!("<!-- B+, B, or B--->");
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
    fn consume_reference() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut reader = Reader::new($content);
                let value = reader.capture(|reader| reader.consume_reference());
                assert_eq!(value.unwrap(), $value);
            });
        );

        test!("&#42; foo", "&#42;");
        test!("&#x42aB; foo", "&#x42aB;");
        test!("&foo; bar", "&foo;");

        macro_rules! test(
            ($content:expr) => ({
                let mut reader = Reader::new($content);
                assert!(!reader.consume_reference());
            });
        );

        test!(" &#42; foo");
        test!("#42; foo");
        test!("&42; foo");
        test!("&#42 foo");
        test!("&#x42z; foo");
        test!("&foo bar");
        test!("foo; bar");
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

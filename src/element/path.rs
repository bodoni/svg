//! The [path][1] element.
//!
//! [1]: https://www.w3.org/TR/SVG/paths.html#PathElement

use Number;
use parser::{Error, Result};
use reader::{Input, Reader};

node! {
    #[doc = "A path element."]
    pub Path("path")
}

/// A data attribute.
///
/// http://www.w3.org/TR/SVG/paths.html#PathData
#[derive(Clone, Debug)]
pub struct Data {
    commands: Vec<Command>,
}

/// A command.
#[derive(Clone, Debug)]
pub enum Command {
    /// [Establish][1] a new current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataMovetoCommands
    MoveTo(Position, Vec<Number>),

    /// [End][1] the current subpath.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataClosePathCommand
    ClosePath,

    /// [Draw][1] straight lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    LineTo(Position, Vec<Number>),

    /// [Draw][1] horizontal lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    HorizontalLineTo(Position, Vec<Number>),

    /// [Draw][1] vertical lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    VerticalLineTo(Position, Vec<Number>),

    /// [Draw][1] a cubic Bézier curve.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    CurveTo(Position, Vec<Number>),

    /// [Draw][1] a cubic Bézier curve assuming the first control point to be
    /// the reflection of the second control point on the previous command
    /// relative to the current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    SmoothCurveTo(Position, Vec<Number>),

    /// [Draw][1] a quadratic Bézier curve.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    QuadraticBezierCurveTo(Position, Vec<Number>),

    /// [Draw][1] a quadratic Bézier curve assuming the control point to be the
    /// reflection of the control point on the previous command relative to the
    /// current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    SmoothQuadraticBezierCurveTo(Position, Vec<Number>),

    /// [Draw][1] an elliptical arc.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    EllipticalArc(Position, Vec<Number>),
}

/// An interpretation of coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    /// Absolute.
    Absolute,
    /// Ralative.
    Relative,
}

impl Data {
    /// Parse a data attribute.
    #[inline]
    pub fn parse<'l, T: Input<'l>>(input: T) -> Result<Self> {
        Parser::new(input).process()
    }

    /// Return an iterator over the commands.
    #[inline]
    pub fn iter(&self) -> ::std::slice::Iter<Command> {
        self.commands.iter()
    }
}

struct Parser<'l> {
    reader: Reader<'l>,
}

macro_rules! raise(
    ($parser:expr, $($arg:tt)*) => ({
        let (line, column) = $parser.reader.position();
        return Err(Error { line: line, column: column, message: format!($($arg)*) })
    });
);

impl<'l> Parser<'l> {
    #[inline]
    fn new<T: Input<'l>>(input: T) -> Self {
        Parser { reader: Reader::new(input) }
    }

    fn process(&mut self) -> Result<Data> {
        let mut commands = Vec::new();
        loop {
            self.reader.consume_whitespace();
            match try!(self.read_command()) {
                Some(command) => commands.push(command),
                _ => break,
            }
        }
        Ok(Data { commands: commands })
    }

    fn read_command(&mut self) -> Result<Option<Command>> {
        use self::Command::*;
        use self::Position::*;

        let name = match self.reader.next() {
            Some(name) => match name {
                'A'...'Z' | 'a'...'z' => name,
                _ => raise!(self, "expected a path command"),
            },
            _ => return Ok(None),
        };
        self.reader.consume_whitespace();
        let parameters = try!(self.read_parameters());
        Ok(Some(match name {
            'M' => MoveTo(Absolute, parameters),
            'm' => MoveTo(Relative, parameters),

            'Z' | 'z' => ClosePath,

            'L' => LineTo(Absolute, parameters),
            'l' => LineTo(Relative, parameters),

            'H' => HorizontalLineTo(Absolute, parameters),
            'h' => HorizontalLineTo(Relative, parameters),

            'V' => VerticalLineTo(Absolute, parameters),
            'v' => VerticalLineTo(Relative, parameters),

            'C' => CurveTo(Absolute, parameters),
            'c' => CurveTo(Relative, parameters),

            'S' => SmoothCurveTo(Absolute, parameters),
            's' => SmoothCurveTo(Relative, parameters),

            'Q' => QuadraticBezierCurveTo(Absolute, parameters),
            'q' => QuadraticBezierCurveTo(Relative, parameters),

            'T' => SmoothQuadraticBezierCurveTo(Absolute, parameters),
            't' => SmoothQuadraticBezierCurveTo(Relative, parameters),

            'A' => EllipticalArc(Absolute, parameters),
            'a' => EllipticalArc(Relative, parameters),

            _ => raise!(self, "found an unknown path command '{}'", name),
        }))
    }

    fn read_parameters(&mut self) -> Result<Vec<Number>> {
        let mut parameters = Vec::new();
        loop {
            match try!(self.read_number()) {
                Some(number) => parameters.push(number),
                _ => break,
            }
            self.reader.consume_whitespace();
            self.reader.consume_any(",");
        }
        Ok(parameters)
    }

    pub fn read_number(&mut self) -> Result<Option<Number>> {
        self.reader.consume_whitespace();
        let number = self.reader.capture(|reader| {
            reader.consume_char('-');
            reader.consume_digits();
            reader.consume_char('.');
            reader.consume_digits();
        }).and_then(|number| Some(String::from(number)));
        match number {
            Some(number) => match (&number).parse() {
                Ok(number) => Ok(Some(number)),
                Err(_) => raise!(self, "failed to parse a number '{}'", number),
            },
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Data, Parser};
    use super::Command::*;
    use super::Position::*;

    #[test]
    fn data_parse() {
        let data = Data::parse("M1,2 l3,4").unwrap();

        assert_eq!(data.commands.len(), 2);

        match data.commands[0] {
            MoveTo(Absolute, ref parameters) => assert_eq!(*parameters, vec![1.0, 2.0]),
            _ => assert!(false),
        }
        match data.commands[1] {
            LineTo(Relative, ref parameters) => assert_eq!(*parameters, vec![3.0, 4.0]),
            _ => assert!(false),
        }
    }

    #[test]
    fn parser_read_command() {
        macro_rules! run(
            ($input:expr) => ({
                let mut parser = Parser::new($input);
                parser.read_command().unwrap().unwrap()
            });
        );

        macro_rules! test(
            ($input:expr, $command:ident, $position:ident, $parameters:expr) => (
                match run!($input) {
                    $command($position, parameters) => assert_eq!(parameters, $parameters),
                    _ => assert!(false),
                }
            );
            ($input:expr, $command:ident) => (
                match run!($input) {
                    $command => {},
                    _ => assert!(false),
                }
            );
        );

        test!("M4,2", MoveTo, Absolute, vec![4.0, 2.0]);
        test!("m4,\n2", MoveTo, Relative, vec![4.0, 2.0]);

        test!("Z", ClosePath);
        test!("z", ClosePath);

        test!("L7, 8  9", LineTo, Absolute, vec![7.0, 8.0, 9.0]);
        test!("l 7,8 \n9", LineTo, Relative, vec![7.0, 8.0, 9.0]);

        test!("H\t6,9", HorizontalLineTo, Absolute, vec![6.0, 9.0]);
        test!("h6,  \t9", HorizontalLineTo, Relative, vec![6.0, 9.0]);

        test!("V2.1,-3", VerticalLineTo, Absolute, vec![2.1, -3.0]);
        test!("v\n2.1 -3", VerticalLineTo, Relative, vec![2.1, -3.0]);

        test!("C0,1 0,2", CurveTo, Absolute, vec![0.0, 1.0, 0.0, 2.0]);
        test!("c0 ,1 0,  2", CurveTo, Relative, vec![0.0, 1.0, 0.0, 2.0]);

        test!("S42,0", SmoothCurveTo, Absolute, vec![42.0, 0.0]);
        test!("s \t 42,0", SmoothCurveTo, Relative, vec![42.0, 0.0]);

        test!("Q90.5 0", QuadraticBezierCurveTo, Absolute, vec![90.5, 0.0]);
        test!("q90.5\n, 0", QuadraticBezierCurveTo, Relative, vec![90.5, 0.0]);

        test!("T-1", SmoothQuadraticBezierCurveTo, Absolute, vec![-1.0]);
        test!("t -1", SmoothQuadraticBezierCurveTo, Relative, vec![-1.0]);

        test!("A2.6,0 -7", EllipticalArc, Absolute, vec![2.6, 0.0, -7.0]);
        test!("a 2.6 ,0 -7", EllipticalArc, Relative, vec![2.6, 0.0, -7.0]);
    }

    #[test]
    fn parser_read_parameters() {
        let mut parser = Parser::new("1,2 3,4 5 6.7");
        let parameters = parser.read_parameters().unwrap();
        assert_eq!(parameters, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.7]);
    }

    #[test]
    fn parser_read_number() {
        let texts = vec!["-1", "3", "3.14"];
        let numbers = vec![-1.0, 3.0, 3.14];
        for (&input, &number) in texts.iter().zip(numbers.iter()) {
            let mut parser = Parser::new(input);
            assert_eq!(parser.read_number().unwrap().unwrap(), number);
        }
    }
}

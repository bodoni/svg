use std::iter::Peekable;
use std::str::CharIndices;

use {Error, Result};

/// The outline of a shape.
///
/// http://www.w3.org/TR/SVG/paths.html
pub struct Path;

/// The data attribute of a path.
///
/// http://www.w3.org/TR/SVG/paths.html#PathData
pub struct Data {
    /// A series of path commands.
    pub commands: Vec<Command>,
}

/// A command used to draw a path.
#[derive(Debug)]
pub enum Command {
    /// [Establish][1] a new current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataMovetoCommands
    MoveTo(Positioning, Vec<f64>),

    /// [End][1] the current subpath.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataClosePathCommand
    ClosePath,

    /// [Draw][1] straight lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    LineTo(Positioning, Vec<f64>),

    /// [Draw][1] horizontal lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    HorizontalLineTo(Positioning, Vec<f64>),

    /// [Draw][1] vertical lines.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataLinetoCommands
    VerticalLineTo(Positioning, Vec<f64>),

    /// [Draw][1] a cubic Bézier curve.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    CurveTo(Positioning, Vec<f64>),

    /// [Draw][1] a cubic Bézier curve assuming the first control point to be
    /// the reflection of the second control point on the previous command
    /// relative to the current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    SmoothCurveTo(Positioning, Vec<f64>),

    /// [Draw][1] a quadratic Bézier curve.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    QuadraticBezierCurveTo(Positioning, Vec<f64>),

    /// [Draw][1] a quadratic Bézier curve assuming the control point to be the
    /// reflection of the control point on the previous command relative to the
    /// current point.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    SmoothQuadraticBezierCurveTo(Positioning, Vec<f64>),

    /// [Draw][1] an elliptical arc.
    ///
    /// [1]: http://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    EllipticalArc(Positioning, Vec<f64>),
}

/// An attribute of a command indicating whether the coordinates of the command
/// are absolute or relative to the current position.
#[derive(Debug)]
pub enum Positioning {
    Absolute,
    Relative,
}

impl Data {
    /// Parse the data attribute of a path.
    #[inline]
    pub fn parse(line: &str) -> Result<Data> {
        DataParser::new(line).process()
    }
}

struct DataParser<'s> {
    line: &'s str,
    position: usize,
    cursor: Peekable<CharIndices<'s>>,
}

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(result) => result,
            Err(error) => return Err(error),
        }
    )
);

macro_rules! raise(
    ($parser:expr, $($arg:tt)*) => (
        return Err(Error {
            line: 1,
            column: $parser.position,
            message: format!($($arg)*),
        })
    );
);

impl<'s> DataParser<'s> {
    fn new(line: &'s str) -> DataParser<'s> {
        DataParser {
            line: line,
            position: 0,
            cursor: line.char_indices().peekable(),
        }
    }

    fn process(&mut self) -> Result<Data> {
        let mut commands = Vec::new();

        loop {
            self.skip_whitespace();

            match ok!(self.read_command()) {
                Some(command) => commands.push(command),
                _ => break,
            }
        }

        Ok(Data {
            commands: commands,
        })
    }

    fn read_command(&mut self) -> Result<Option<Command>> {
        use self::Command::*;
        use self::Positioning::*;

        let name = match self.next() {
            Some(name) => match name {
                'A'...'Z' | 'a'...'z' => name,
                _ => raise!(self, "expected a path command"),
            },
            _ => return Ok(None),
        };

        self.skip_whitespace();

        let params = ok!(self.read_parameters());

        Ok(Some(match name {
            'M' => MoveTo(Absolute, params),
            'm' => MoveTo(Relative, params),

            'Z' | 'z' => ClosePath,

            'L' => LineTo(Absolute, params),
            'l' => LineTo(Relative, params),

            'H' => HorizontalLineTo(Absolute, params),
            'h' => HorizontalLineTo(Relative, params),

            'V' => VerticalLineTo(Absolute, params),
            'v' => VerticalLineTo(Relative, params),

            'C' => CurveTo(Absolute, params),
            'c' => CurveTo(Relative, params),

            'S' => SmoothCurveTo(Absolute, params),
            's' => SmoothCurveTo(Relative, params),

            'Q' => QuadraticBezierCurveTo(Absolute, params),
            'q' => QuadraticBezierCurveTo(Relative, params),

            'T' => SmoothQuadraticBezierCurveTo(Absolute, params),
            't' => SmoothQuadraticBezierCurveTo(Relative, params),

            'A' => EllipticalArc(Absolute, params),
            'a' => EllipticalArc(Relative, params),

            _ => raise!(self, "found an unknown path command '{}'", name),
        }))
    }

    fn read_parameters(&mut self) -> Result<Vec<f64>> {
        let mut params = Vec::new();

        loop {
            match ok!(self.read_number()) {
                Some(number) => params.push(number),
                _ => break,
            }

            self.skip_whitespace();
            self.skip(",");
        }

        Ok(params)
    }

    fn read_number(&mut self) -> Result<Option<f64>> {
        self.skip_whitespace();

        let start = self.position;

        self.skip("-");
        self.skip_digits();
        self.skip(".");
        self.skip_digits();

        let end = self.position;

        if start == end {
            return Ok(None)
        }

        let number = &self.line[start..end];

        match number.parse() {
            Ok(number) => Ok(Some(number)),
            Err(_) => raise!(self, "failed to parse a number '{}'", number),
        }
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        self.skip(" \t")
    }

    #[inline]
    fn skip_digits(&mut self) {
        self.skip("0123456789")
    }

    fn skip(&mut self, chars: &str) {
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
    fn next(&mut self) -> Option<char> {
        match self.cursor.next() {
            Some((_, c)) => {
                self.position += 1;
                Some(c)
            },
            _ => None,
        }
    }

    #[inline]
    fn peek(&mut self) -> Option<char> {
        match self.cursor.peek() {
            Some(&(_, c)) => Some(c),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Data, DataParser};
    use super::Command::*;
    use super::Positioning::*;

    #[test]
    fn data_parser() {
        let data = Data::parse("M1,2 l3,4").ok().unwrap();

        assert_eq!(data.commands.len(), 2);

        match data.commands[0] {
            MoveTo(Absolute, ref params) => assert_eq!(*params, vec![1.0, 2.0]),
            _ => assert!(false),
        }

        match data.commands[1] {
            LineTo(Relative, ref params) => assert_eq!(*params, vec![3.0, 4.0]),
            _ => assert!(false),
        }
    }

    #[test]
    fn data_parser_read_command() {
        macro_rules! run(
            ($line:expr) => ({
                let mut parser = DataParser::new($line);
                parser.read_command().ok().unwrap().unwrap()
            });
        );

        macro_rules! test(
            ($line:expr, $command:ident, $positioning:ident, $params:expr) => (
                match run!($line) {
                    $command($positioning, params) => assert_eq!(params, $params),
                    _ => assert!(false),
                }
            );
            ($line:expr, $command:ident) => (
                match run!($line) {
                    $command => {},
                    _ => assert!(false),
                }
            );
        );

        test!("M4,2", MoveTo, Absolute, vec![4.0, 2.0]);
        test!("m4,2", MoveTo, Relative, vec![4.0, 2.0]);

        test!("Z", ClosePath);
        test!("z", ClosePath);

        test!("L7, 8  9", LineTo, Absolute, vec![7.0, 8.0, 9.0]);
        test!("l 7,8 9", LineTo, Relative, vec![7.0, 8.0, 9.0]);

        test!("H\t6,9", HorizontalLineTo, Absolute, vec![6.0, 9.0]);
        test!("h6,  \t9", HorizontalLineTo, Relative, vec![6.0, 9.0]);

        test!("V2.1,-3", VerticalLineTo, Absolute, vec![2.1, -3.0]);
        test!("v2.1 -3", VerticalLineTo, Relative, vec![2.1, -3.0]);

        test!("C0,1 0,2", CurveTo, Absolute, vec![0.0, 1.0, 0.0, 2.0]);
        test!("c0 ,1 0,  2", CurveTo, Relative, vec![0.0, 1.0, 0.0, 2.0]);

        test!("S42,0", SmoothCurveTo, Absolute, vec![42.0, 0.0]);
        test!("s \t 42,0", SmoothCurveTo, Relative, vec![42.0, 0.0]);

        test!("Q90.5 0", QuadraticBezierCurveTo, Absolute, vec![90.5, 0.0]);
        test!("q90.5, 0", QuadraticBezierCurveTo, Relative, vec![90.5, 0.0]);

        test!("T-1", SmoothQuadraticBezierCurveTo, Absolute, vec![-1.0]);
        test!("t -1", SmoothQuadraticBezierCurveTo, Relative, vec![-1.0]);

        test!("A2.6,0 -7", EllipticalArc, Absolute, vec![2.6, 0.0, -7.0]);
        test!("a 2.6 ,0 -7", EllipticalArc, Relative, vec![2.6, 0.0, -7.0]);
    }

    #[test]
    fn data_parser_read_parameters() {
        let mut parser = DataParser::new("1,2 3,4 5 6.7");
        let params = parser.read_parameters().ok().unwrap();
        assert_eq!(params, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.7]);
    }

    #[test]
    fn data_parser_read_number() {
        let lines = vec!["-1", "3", "3.14"];
        let numbers = vec![-1.0, 3.0, 3.14];

        for (line, &number) in lines.iter().zip(numbers.iter()) {
            let mut parser = DataParser::new(line);
            assert_eq!(parser.read_number().ok().unwrap().unwrap(), number);
        }
    }

    #[test]
    fn data_parser_skip_whitespace() {
        let mut parser = DataParser::new(" \t    m ");
        parser.skip_whitespace();
        assert_eq!(parser.position, 6);
    }
}

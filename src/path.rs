use {Error, Result};
use reader::Reader;

/// The outline of a shape.
///
/// http://www.w3.org/TR/SVG/paths.html
pub struct Path;

/// The data attribute of a path.
///
/// http://www.w3.org/TR/SVG/paths.html#PathData
pub struct Data {
    commands: Vec<Command>,
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
    pub fn parse(text: &str) -> Result<Data> {
        Parser::new(text).process()
    }

    /// Return an iterator over the commands of the path.
    #[inline]
    pub fn iter(&self) -> ::std::slice::Iter<Command> {
        self.commands.iter()
    }
}

struct Parser<'s> {
    reader: Reader<'s>,
}

macro_rules! raise(
    ($parser:expr, $($arg:tt)*) => ({
        let (line, column) = $parser.reader.position();
        return Err(Error {
            line: line,
            column: column,
            message: format!($($arg)*),
        })
    });
);

impl<'s> Parser<'s> {
    #[inline]
    fn new(text: &'s str) -> Parser<'s> {
        Parser {
            reader: Reader::new(text),
        }
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

        Ok(Data {
            commands: commands,
        })
    }

    fn read_command(&mut self) -> Result<Option<Command>> {
        use self::Command::*;
        use self::Positioning::*;

        let name = match self.reader.next() {
            Some(name) => match name {
                'A'...'Z' | 'a'...'z' => name,
                _ => raise!(self, "expected a path command"),
            },
            _ => return Ok(None),
        };

        self.reader.consume_whitespace();

        let params = try!(self.read_parameters());

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
            match try!(self.read_number()) {
                Some(number) => params.push(number),
                _ => break,
            }

            self.reader.consume_whitespace();
            self.reader.consume_any(",");
        }

        Ok(params)
    }

    pub fn read_number(&mut self) -> Result<Option<f64>> {
        self.reader.consume_whitespace();

        let number = String::from_str(self.reader.capture(|reader| {
            reader.consume_any("-");
            reader.consume_digits();
            reader.consume_any(".");
            reader.consume_digits();
        }));

        if number.is_empty() {
            return Ok(None)
        }

        match (&number).parse() {
            Ok(number) => Ok(Some(number)),
            Err(_) => raise!(self, "failed to parse a number '{}'", number),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Data, Parser};
    use super::Command::*;
    use super::Positioning::*;

    #[test]
    fn data_parse() {
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
    fn parser_read_command() {
        macro_rules! run(
            ($text:expr) => ({
                let mut parser = Parser::new($text);
                parser.read_command().ok().unwrap().unwrap()
            });
        );

        macro_rules! test(
            ($text:expr, $command:ident, $positioning:ident, $params:expr) => (
                match run!($text) {
                    $command($positioning, params) => assert_eq!(params, $params),
                    _ => assert!(false),
                }
            );
            ($text:expr, $command:ident) => (
                match run!($text) {
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
        let params = parser.read_parameters().ok().unwrap();
        assert_eq!(params, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.7]);
    }

    #[test]
    fn parser_read_number() {
        let texts = vec!["-1", "3", "3.14"];
        let numbers = vec![-1.0, 3.0, 3.14];

        for (text, &number) in texts.iter().zip(numbers.iter()) {
            let mut parser = Parser::new(text);
            assert_eq!(parser.read_number().ok().unwrap().unwrap(), number);
        }
    }
}

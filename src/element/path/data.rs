use std::borrow::Cow;

use element::path::command::{Command, Parameters, Position};
use error::Parse as Error;
use node::Value;
use reader::Reader;
use result::Parse as Result;

/// A [data][1] attribute.
///
/// [1]: http://www.w3.org/TR/SVG/paths.html#PathData
#[derive(Clone, Debug, Default)]
pub struct Data(Vec<Command>);

struct Parser<'l> {
    reader: Reader<'l>,
}

impl Data {
    /// Create data.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Parse a data attribute.
    #[inline]
    pub fn parse<'l, T: Into<Cow<'l, str>>>(content: T) -> Result<Self> {
        Parser::new(content).process()
    }
}

macro_rules! command(
    ($data:ident, $command:ident) => (
        $data.0.push(Command::$command);
    );
    ($data:ident, $command:ident, $position:ident, $parameters:ident) => (
        $data.0.push(Command::$command(Position::$position, $parameters.into()));
    );
);

impl Data {
    /// Add an aboslute `Command::Move` command.
    pub fn move_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, Move, Absolute, parameters);
        self
    }

    /// Add a relative `Command::Move` command.
    pub fn move_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, Move, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::Line` command.
    pub fn line_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, Line, Absolute, parameters);
        self
    }

    /// Add a relative `Command::Line` command.
    pub fn line_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, Line, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::HorizontalLine` command.
    pub fn horizontal_line_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, HorizontalLine, Absolute, parameters);
        self
    }

    /// Add a relative `Command::HorizontalLine` command.
    pub fn horizontal_line_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, HorizontalLine, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::VerticalLine` command.
    pub fn vertical_line_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, VerticalLine, Absolute, parameters);
        self
    }

    /// Add a relative `Command::VerticalLine` command.
    pub fn vertical_line_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, VerticalLine, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::QuadraticCurve` command.
    pub fn quadratic_curve_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, QuadraticCurve, Absolute, parameters);
        self
    }

    /// Add a relative `Command::QuadraticCurve` command.
    pub fn quadratic_curve_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, QuadraticCurve, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::SmoothQuadraticCurve` command.
    pub fn smooth_quadratic_curve_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, SmoothQuadraticCurve, Absolute, parameters);
        self
    }

    /// Add a relative `Command::SmoothQuadraticCurve` command.
    pub fn smooth_quadratic_curve_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, SmoothQuadraticCurve, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::CubicCurve` command.
    pub fn cubic_curve_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, CubicCurve, Absolute, parameters);
        self
    }

    /// Add a relative `Command::CubicCurve` command.
    pub fn cubic_curve_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, CubicCurve, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::SmoothCubicCurve` command.
    pub fn smooth_cubic_curve_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, SmoothCubicCurve, Absolute, parameters);
        self
    }

    /// Add a relative `Command::SmoothCubicCurve` command.
    pub fn smooth_cubic_curve_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, SmoothCubicCurve, Relative, parameters);
        self
    }

    /// Add an aboslute `Command::EllipticalArc` command.
    pub fn elliptical_arc_to<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, EllipticalArc, Absolute, parameters);
        self
    }

    /// Add a relative `Command::EllipticalArc` command.
    pub fn elliptical_arc_by<T: Parameters>(mut self, parameters: T) -> Self {
        command!(self, EllipticalArc, Relative, parameters);
        self
    }

    /// Add a `Command::Close` command.
    pub fn close(mut self) -> Self {
        command!(self, Close);
        self
    }
}

deref! { Data::0 => [Command] }

impl From<Vec<Command>> for Data {
    #[inline]
    fn from(commands: Vec<Command>) -> Self {
        Data(commands)
    }
}

impl From<Data> for Vec<Command> {
    #[inline]
    fn from(Data(commands): Data) -> Self {
        commands
    }
}

impl Value for Data {
    fn into(self) -> String {
        "".to_string()
    }
}

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Err(Error::new($parser.reader.position(), format!($($argument)*)));
    );
);

impl<'l> Parser<'l> {
    #[inline]
    fn new<T: Into<Cow<'l, str>>>(content: T) -> Self {
        Parser { reader: Reader::new(content) }
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
        Ok(Data(commands))
    }

    fn read_command(&mut self) -> Result<Option<Command>> {
        use element::path::command::Command::*;
        use element::path::command::Position::*;

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
            'M' => Move(Absolute, parameters),
            'm' => Move(Relative, parameters),

            'L' => Line(Absolute, parameters),
            'l' => Line(Relative, parameters),

            'H' => HorizontalLine(Absolute, parameters),
            'h' => HorizontalLine(Relative, parameters),

            'V' => VerticalLine(Absolute, parameters),
            'v' => VerticalLine(Relative, parameters),

            'Q' => QuadraticCurve(Absolute, parameters),
            'q' => QuadraticCurve(Relative, parameters),

            'T' => SmoothQuadraticCurve(Absolute, parameters),
            't' => SmoothQuadraticCurve(Relative, parameters),

            'C' => CubicCurve(Absolute, parameters),
            'c' => CubicCurve(Relative, parameters),

            'S' => SmoothCubicCurve(Absolute, parameters),
            's' => SmoothCubicCurve(Relative, parameters),

            'A' => EllipticalArc(Absolute, parameters),
            'a' => EllipticalArc(Relative, parameters),

            'Z' | 'z' => Close,

            _ => raise!(self, "found an unknown path command '{}'", name),
        }))
    }

    fn read_parameters(&mut self) -> Result<Vec<f32>> {
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

    pub fn read_number(&mut self) -> Result<Option<f32>> {
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
    use element::path::Command::*;
    use element::path::Position::*;
    use super::{Data, Parser};

    #[test]
    fn data_parse() {
        let data = Data::parse("M1,2 l3,4").unwrap();

        assert_eq!(data.len(), 2);

        match data[0] {
            Move(Absolute, ref parameters) => assert_eq!(*parameters, vec![1.0, 2.0]),
            _ => assert!(false),
        }
        match data[1] {
            Line(Relative, ref parameters) => assert_eq!(*parameters, vec![3.0, 4.0]),
            _ => assert!(false),
        }
    }

    #[test]
    fn parser_read_command() {
        macro_rules! run(
            ($content:expr) => ({
                let mut parser = Parser::new($content);
                parser.read_command().unwrap().unwrap()
            });
        );

        macro_rules! test(
            ($content:expr, $command:ident, $position:ident, $parameters:expr) => (
                match run!($content) {
                    $command($position, parameters) => assert_eq!(parameters, $parameters),
                    _ => assert!(false),
                }
            );
            ($content:expr, $command:ident) => (
                match run!($content) {
                    $command => {},
                    _ => assert!(false),
                }
            );
        );

        test!("M4,2", Move, Absolute, vec![4.0, 2.0]);
        test!("m4,\n2", Move, Relative, vec![4.0, 2.0]);

        test!("L7, 8  9", Line, Absolute, vec![7.0, 8.0, 9.0]);
        test!("l 7,8 \n9", Line, Relative, vec![7.0, 8.0, 9.0]);

        test!("H\t6,9", HorizontalLine, Absolute, vec![6.0, 9.0]);
        test!("h6,  \t9", HorizontalLine, Relative, vec![6.0, 9.0]);

        test!("V2.1,-3", VerticalLine, Absolute, vec![2.1, -3.0]);
        test!("v\n2.1 -3", VerticalLine, Relative, vec![2.1, -3.0]);

        test!("Q90.5 0", QuadraticCurve, Absolute, vec![90.5, 0.0]);
        test!("q90.5\n, 0", QuadraticCurve, Relative, vec![90.5, 0.0]);

        test!("T-1", SmoothQuadraticCurve, Absolute, vec![-1.0]);
        test!("t -1", SmoothQuadraticCurve, Relative, vec![-1.0]);

        test!("C0,1 0,2", CubicCurve, Absolute, vec![0.0, 1.0, 0.0, 2.0]);
        test!("c0 ,1 0,  2", CubicCurve, Relative, vec![0.0, 1.0, 0.0, 2.0]);

        test!("S42,0", SmoothCubicCurve, Absolute, vec![42.0, 0.0]);
        test!("s \t 42,0", SmoothCubicCurve, Relative, vec![42.0, 0.0]);

        test!("A2.6,0 -7", EllipticalArc, Absolute, vec![2.6, 0.0, -7.0]);
        test!("a 2.6 ,0 -7", EllipticalArc, Relative, vec![2.6, 0.0, -7.0]);

        test!("Z", Close);
        test!("z", Close);
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
        for (&content, &number) in texts.iter().zip(numbers.iter()) {
            let mut parser = Parser::new(content);
            assert_eq!(parser.read_number().unwrap().unwrap(), number);
        }
    }
}

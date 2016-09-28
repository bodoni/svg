use std::ops::Deref;

use node::Value;
use parser::{Error, Reader, Result};
use super::{Command, Number, Parameters, Position};

/// A [data][1] attribute.
///
/// [1]: https://www.w3.org/TR/SVG/paths.html#PathData
#[derive(Clone, Default)]
pub struct Data(Vec<Command>);

struct Parser<'l> {
    reader: Reader<'l>,
}

impl Data {
    /// Create a data attribute.
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Parse a data attribute.
    #[inline]
    pub fn parse(content: &str) -> Result<Self> {
        Parser::new(content).process()
    }
}

macro_rules! implement {
    (@one #[$doc:meta] fn $method:ident($command:ident, $position:ident)) => (
        #[$doc]
        pub fn $method<T>(mut self, parameters: T) -> Self where T: Into<Parameters> {
            self.0.push(Command::$command(Position::$position, parameters.into()));
            self
        }
    );
    (@one #[$doc:meta] fn $method:ident($command:ident)) => (
        #[$doc]
        pub fn $method(mut self) -> Self {
            self.0.push(Command::$command);
            self
        }
    );
    ($(#[$doc:meta] fn $method:ident($($argument:tt)*))*) => (
        impl Data {
            $(implement! { @one #[$doc] fn $method($($argument)*) })*
        }
    );
}

implement! {
    #[doc = "Add an absolute `Command::Move` command."]
    fn move_to(Move, Absolute)

    #[doc = "Add a relative `Command::Move` command."]
    fn move_by(Move, Relative)

    #[doc = "Add an absolute `Command::Line` command."]
    fn line_to(Line, Absolute)

    #[doc = "Add a relative `Command::Line` command."]
    fn line_by(Line, Relative)

    #[doc = "Add an absolute `Command::HorizontalLine` command."]
    fn horizontal_line_to(HorizontalLine, Absolute)

    #[doc = "Add a relative `Command::HorizontalLine` command."]
    fn horizontal_line_by(HorizontalLine, Relative)

    #[doc = "Add an absolute `Command::VerticalLine` command."]
    fn vertical_line_to(VerticalLine, Absolute)

    #[doc = "Add a relative `Command::VerticalLine` command."]
    fn vertical_line_by(VerticalLine, Relative)

    #[doc = "Add an absolute `Command::QuadraticCurve` command."]
    fn quadratic_curve_to(QuadraticCurve, Absolute)

    #[doc = "Add a relative `Command::QuadraticCurve` command."]
    fn quadratic_curve_by(QuadraticCurve, Relative)

    #[doc = "Add an absolute `Command::SmoothQuadraticCurve` command."]
    fn smooth_quadratic_curve_to(SmoothQuadraticCurve, Absolute)

    #[doc = "Add a relative `Command::SmoothQuadraticCurve` command."]
    fn smooth_quadratic_curve_by(SmoothQuadraticCurve, Relative)

    #[doc = "Add an absolute `Command::CubicCurve` command."]
    fn cubic_curve_to(CubicCurve, Absolute)

    #[doc = "Add a relative `Command::CubicCurve` command."]
    fn cubic_curve_by(CubicCurve, Relative)

    #[doc = "Add an absolute `Command::SmoothCubicCurve` command."]
    fn smooth_cubic_curve_to(SmoothCubicCurve, Absolute)

    #[doc = "Add a relative `Command::SmoothCubicCurve` command."]
    fn smooth_cubic_curve_by(SmoothCubicCurve, Relative)

    #[doc = "Add an absolute `Command::EllipticalArc` command."]
    fn elliptical_arc_to(EllipticalArc, Absolute)

    #[doc = "Add a relative `Command::EllipticalArc` command."]
    fn elliptical_arc_by(EllipticalArc, Relative)

    #[doc = "Add a `Command::Close` command."]
    fn close(Close)
}

impl Deref for Data {
    type Target = [Command];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

impl From<Data> for Value {
    #[inline]
    fn from(Data(mut inner): Data) -> Self {
        inner.drain(..).map(|value| String::from(value)).collect::<Vec<_>>().join(" ").into()
    }
}

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Err(Error::new($parser.reader.position(), format!($($argument)*)));
    );
);

impl<'l> Parser<'l> {
    #[inline]
    fn new(content: &'l str) -> Self {
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
        use super::Command::*;
        use super::Position::*;

        let name = match self.reader.next() {
            Some(name) => match name {
                'A'...'Z' | 'a'...'z' => name,
                _ => raise!(self, "expected a path command"),
            },
            _ => return Ok(None),
        };
        self.reader.consume_whitespace();
        let parameters = try!(self.read_parameters()).into();
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

    fn read_parameters(&mut self) -> Result<Vec<Number>> {
        let mut parameters = Vec::new();
        loop {
            match try!(self.read_number()) {
                Some(number) => parameters.push(number),
                _ => break,
            }
            self.reader.consume_whitespace();
            self.reader.consume_char(',');
        }
        Ok(parameters)
    }

    pub fn read_number(&mut self) -> Result<Option<Number>> {
        self.reader.consume_whitespace();
        let number = self.reader.capture(|reader| {
            reader.consume_number();
        }).and_then(|number| Some(String::from(number)));
        match number {
            Some(number) => match (&number).parse() {
                Ok(number) => Ok(Some(number)),
                _ => raise!(self, "failed to parse a number '{}'", number),
            },
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use node::Value;
    use super::{Data, Parser};
    use super::super::Command::*;
    use super::super::Position::*;

    #[test]
    fn data_into_value() {
        let data = Data::new().line_to((1, 2)).cubic_curve_by((1, 2.5, 3, 4, 5, 6)).close();
        assert_eq!(Value::from(data).to_string(), "L1,2 c1,2.5,3,4,5,6 z");
    }

    #[test]
    fn data_parse() {
        let data = Data::parse("M1,2 l3,4").unwrap();
        assert_eq!(data.len(), 2);
        match data[0] {
            Move(Absolute, ref parameters) => assert_eq!(&parameters[..], &[1.0, 2.0]),
            _ => unreachable!(),
        }
        match data[1] {
            Line(Relative, ref parameters) => assert_eq!(&parameters[..], &[3.0, 4.0]),
            _ => unreachable!(),
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
                    $command($position, parameters) => assert_eq!(&parameters[..], $parameters),
                    _ => unreachable!(),
                }
            );
            ($content:expr, $command:ident) => (
                match run!($content) {
                    $command => {},
                    _ => unreachable!(),
                }
            );
        );

        test!("M4,2", Move, Absolute, &[4.0, 2.0]);
        test!("m4,\n2", Move, Relative, &[4.0, 2.0]);

        test!("L7, 8  9", Line, Absolute, &[7.0, 8.0, 9.0]);
        test!("l 7,8 \n9", Line, Relative, &[7.0, 8.0, 9.0]);

        test!("H\t6,9", HorizontalLine, Absolute, &[6.0, 9.0]);
        test!("h6,  \t9", HorizontalLine, Relative, &[6.0, 9.0]);

        test!("V2.1,-3", VerticalLine, Absolute, &[2.1, -3.0]);
        test!("v\n2.1 -3", VerticalLine, Relative, &[2.1, -3.0]);

        test!("Q90.5 0", QuadraticCurve, Absolute, &[90.5, 0.0]);
        test!("q90.5\n, 0", QuadraticCurve, Relative, &[90.5, 0.0]);

        test!("T-1", SmoothQuadraticCurve, Absolute, &[-1.0]);
        test!("t -1", SmoothQuadraticCurve, Relative, &[-1.0]);

        test!("C0,1 0,2", CubicCurve, Absolute, &[0.0, 1.0, 0.0, 2.0]);
        test!("c0 ,1 0,  2", CubicCurve, Relative, &[0.0, 1.0, 0.0, 2.0]);

        test!("S42,0", SmoothCubicCurve, Absolute, &[42.0, 0.0]);
        test!("s \t 42,0", SmoothCubicCurve, Relative, &[42.0, 0.0]);

        test!("A2.6,0 -7", EllipticalArc, Absolute, &[2.6, 0.0, -7.0]);
        test!("a 2.6 ,0 -7", EllipticalArc, Relative, &[2.6, 0.0, -7.0]);

        test!("Z", Close);
        test!("z", Close);
    }

    #[test]
    fn parser_read_parameters() {
        let mut parser = Parser::new("1,2 3,4 5 6.7");
        let parameters = parser.read_parameters().unwrap();
        assert_eq!(&parameters[..], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.7]);
    }

    #[test]
    fn parser_read_number_double() {
        let mut parser = Parser::new("0.30000000000000004");
        assert_eq!(parser.read_number().unwrap().unwrap(), 0.3);
    }

    #[test]
    fn parser_read_number_exponent() {
        let mut parser = Parser::new("1e-4");
        assert_eq!(parser.read_number().unwrap().unwrap(), 1e-4);

        let mut parser = Parser::new("-1E2");
        assert_eq!(parser.read_number().unwrap().unwrap(), -1E2);

        let mut parser = Parser::new("-0.00100E-002");
        assert_eq!(parser.read_number().unwrap().unwrap(), -0.00100E-002);
    }
}

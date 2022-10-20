use std::ops::Deref;

use super::{Command, Number, Parameters, Position};
use crate::node::Value;
use crate::parser::{Error, Reader, Result};

/// A [data][1] attribute.
///
/// [1]: https://www.w3.org/TR/SVG/paths.html#PathData
#[derive(Clone, Debug, Default)]
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

    /// Add a command.
    #[inline]
    pub fn add(mut self, command: Command) -> Self {
        self.0.push(command);
        self
    }
}

macro_rules! implement {
    (@one #[$doc:meta] fn $method:ident($command:ident, $position:ident)) => (
        #[$doc]
        pub fn $method<T>(mut self, parameters: T) -> Self
        where
            T: Into<Parameters>,
        {
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
        inner
            .drain(..)
            .map(String::from)
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    }
}

macro_rules! raise(
    ($parser:expr, $($argument:tt)*) => (
        return Err(Error::new($parser.reader.position(), format!($($argument)*)))
    );
);

impl<'l> Parser<'l> {
    #[inline]
    fn new(content: &'l str) -> Self {
        Parser {
            reader: Reader::new(content),
        }
    }

    fn process(&mut self) -> Result<Data> {
        let mut commands = Vec::new();
        loop {
            self.reader.consume_whitespace();
            match self.read_command()? {
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
                'A'..='Z' | 'a'..='z' => name,
                _ => raise!(self, "expected a path command"),
            },
            _ => return Ok(None),
        };
        self.reader.consume_whitespace();
        Ok(Some(match name {
            'M' => Move(Absolute, self.read_parameters()?.into()),
            'm' => Move(Relative, self.read_parameters()?.into()),

            'L' => Line(Absolute, self.read_parameters()?.into()),
            'l' => Line(Relative, self.read_parameters()?.into()),

            'H' => HorizontalLine(Absolute, self.read_parameters()?.into()),
            'h' => HorizontalLine(Relative, self.read_parameters()?.into()),

            'V' => VerticalLine(Absolute, self.read_parameters()?.into()),
            'v' => VerticalLine(Relative, self.read_parameters()?.into()),

            'Q' => QuadraticCurve(Absolute, self.read_parameters()?.into()),
            'q' => QuadraticCurve(Relative, self.read_parameters()?.into()),

            'T' => SmoothQuadraticCurve(Absolute, self.read_parameters()?.into()),
            't' => SmoothQuadraticCurve(Relative, self.read_parameters()?.into()),

            'C' => CubicCurve(Absolute, self.read_parameters()?.into()),
            'c' => CubicCurve(Relative, self.read_parameters()?.into()),

            'S' => SmoothCubicCurve(Absolute, self.read_parameters()?.into()),
            's' => SmoothCubicCurve(Relative, self.read_parameters()?.into()),

            'A' => EllipticalArc(Absolute, self.read_parameters_elliptical_arc()?.into()),
            'a' => EllipticalArc(Relative, self.read_parameters_elliptical_arc()?.into()),

            'Z' | 'z' => Close,

            _ => raise!(self, "found an unknown path command '{}'", name),
        }))
    }

    fn read_parameters(&mut self) -> Result<Vec<Number>> {
        let mut parameters = Vec::new();

        while let Some(number) = self.read_number()? {
            parameters.push(number);
            self.reader.consume_whitespace();
            self.reader.consume_char(',');
        }
        Ok(parameters)
    }

    fn read_parameters_elliptical_arc(&mut self) -> Result<Vec<Number>> {
        let mut parameters = Vec::new();
        let mut index: usize = 1;

        while let Some(number) = match index % 7 {
            i if i == 4 || i == 5 => self.read_flag()?,
            _ => self.read_number()?,
        } {
            index += 1;
            parameters.push(number);
            self.reader.consume_whitespace();
            self.reader.consume_char(',');
        }
        Ok(parameters)
    }

    fn read_flag(&mut self) -> Result<Option<Number>> {
        self.reader.consume_whitespace();

        match self.reader.next() {
            Some('0') => Ok(Some(0.0)),
            Some('1') => Ok(Some(1.0)),
            _ => raise!(self, "failed to parse a flag in an elliptical arc"),
        }
    }

    pub fn read_number(&mut self) -> Result<Option<Number>> {
        self.reader.consume_whitespace();
        let number = self
            .reader
            .capture(|reader| reader.consume_number())
            .and_then(|number| Some(String::from(number)));
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
    use super::super::Command::*;
    use super::super::Position::*;
    use super::{Data, Parser};
    use crate::node::Value;

    #[test]
    fn data_into_value() {
        let data = Data::new()
            .line_to((1, 2))
            .cubic_curve_by((1, 2.5, 3, 4, 5, 6))
            .close();

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
                    $command => {}
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

        test!(
            "A1 1 2.6,0 0 0 -7",
            EllipticalArc,
            Absolute,
            &[1.0, 1.0, 2.6, 0.0, 0.0, 0.0, -7.0]
        );
        test!(
            "a1 1 2.6,0 0 0 -7",
            EllipticalArc,
            Relative,
            &[1.0, 1.0, 2.6, 0.0, 0.0, 0.0, -7.0]
        );
        test!(
            "a32 32 0 00.03-45.22",
            EllipticalArc,
            Relative,
            &[32.0, 32.0, 0.0, 0.0, 0.0, 0.03, -45.22]
        );
        test!(
            "a48 48 0 1148-48",
            EllipticalArc,
            Relative,
            &[48.0, 48.0, 0.0, 1.0, 1.0, 48.0, -48.0]
        );
        test!(
            "a82.6 82.6 0 0033.48-20.25",
            EllipticalArc,
            Relative,
            &[82.6, 82.6, 0.0, 0.0, 0.0, 33.48, -20.25]
        );
        test!(
            "a82.45 82.45 0 00-20.24 33.47",
            EllipticalArc,
            Relative,
            &[82.45, 82.45, 0.0, 0.0, 0.0, -20.24, 33.47]
        );
        test!(
            "a48 48 0 1148-48 48 48 0 01-48 48",
            EllipticalArc,
            Relative,
            &[48.0, 48.0, 0.0, 1.0, 1.0, 48.0, -48.0, 48.0, 48.0, 0.0, 0.0, 1.0, -48.0, 48.0]
        );
        test!(
            "a48 48 0 1148-48 48 48 0 01-48 48 32 32 0 11.03-45.22",
            EllipticalArc,
            Relative,
            &[
                48.0, 48.0, 0.0, 1.0, 1.0, 48.0, -48.0, 48.0, 48.0, 0.0, 0.0, 1.0, -48.0, 48.0,
                32.0, 32.0, 0.0, 1.0, 1.0, 0.03, -45.22
            ]
        );
        test!(
            "a2.51 2.51 0 01.25.32",
            EllipticalArc,
            Relative,
            &[2.51, 2.51, 0.0, 0.0, 1.0, 0.25, 0.32]
        );
        test!(
            "a1 1 0 00.25.32",
            EllipticalArc,
            Relative,
            &[1., 1., 0.0, 0.0, 0.0, 0.25, 0.32]
        );
        test!(
            "a1 1 0 000.25.32",
            EllipticalArc,
            Relative,
            &[1., 1., 0.0, 0.0, 0.0, 0.25, 0.32]
        );

        test!("Z", Close);
        test!("z", Close);
    }

    #[test]
    fn parser_read_parameters() {
        macro_rules! test(
            ($content:expr, $parameters:expr) => ({
                let mut parser = Parser::new($content);
                let parameters = parser.read_parameters().unwrap();
                assert_eq!(&parameters[..], $parameters);
            });
        );

        test!("1,2 3,4 5 6.7", &[1.0, 2.0, 3.0, 4.0, 5.0, 6.7]);
        test!("4-3.1.3e2.4", &[4.0, -3.1, 0.3e2, 0.4]);
    }

    #[test]
    fn parser_read_parameters_elliptical_arc() {
        macro_rules! test(
            ($content:expr, $parameters:expr) => ({
                let mut parser = Parser::new($content);
                let parameters = parser.read_parameters_elliptical_arc().unwrap();
                assert_eq!(&parameters[..], $parameters);
            });
        );

        test!(
            "32 32 0 00.03-45.22",
            &[32.0, 32.0, 0.0, 0.0, 0.0, 0.03, -45.22]
        );
        test!("48 48 0 1148-48", &[48.0, 48.0, 0.0, 1.0, 1.0, 48.0, -48.0]);
    }

    #[test]
    fn parser_read_number() {
        macro_rules! test(
            ($content:expr, $value:expr) => ({
                let mut parser = Parser::new($content);
                assert_eq!(parser.read_number().unwrap().unwrap(), $value);
            });
        );

        test!("0.30000000000000004", 0.3);
        test!("1e-4", 1e-4);
        test!("-1E2", -1e2);
        test!("-0.00100E-002", -1e-5);
    }
}

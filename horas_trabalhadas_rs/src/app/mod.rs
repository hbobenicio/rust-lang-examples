mod input;

use std::num;
use std::fmt;
use std::convert;
use std::string;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    WrongNumberOfArgs(usize),
    ParsingError(String),
    NotOrdered(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongNumberOfArgs(n) =>
                write!(f, "número errado de argumentos: necessários 1, 2 ou 4. passados: {}", n),
            Error::ParsingError(s) =>
                write!(f, "falha no parsing: {}", s),
            Error::NotOrdered(s) =>
                write!(f, "horários não estão em ordem crescente: {}", s)
        }
    }
}

impl convert::From<string::ParseError> for Error {
    fn from(other: string::ParseError) -> Self {
        Error::ParsingError(other.to_string())
    }
}

impl convert::From<num::ParseIntError> for Error {
    fn from(other: num::ParseIntError) -> Self {
        Error::ParsingError(other.to_string())
    }
}

fn parse_time(str_time: &str) -> Result<u32, Error> {
    let len: usize = str_time.chars().count();
    let has_separator: bool = str_time.chars().nth(2) == Some(':');

    if len != 5 || !has_separator {
        let error_msg = format!("Horário {} não é um formato de hora válida (hh:mm)", str_time);
        return Err(Error::ParsingError(error_msg));
    }

    let parts: Vec<&str> = str_time.split(':').collect();
    let hour: u32 = parts[0].parse()?;
    let min: u32 = parts[1].parse()?;

    if min >= 60 {
        let error_msg = format!("Minutos devem ser menores que 60");
        return Err(Error::ParsingError(error_msg));
    }

    if hour >= 24 {
        let error_msg = format!("Horas trabalhadas devem ser menores que 24");
        return Err(Error::ParsingError(error_msg));
    }

    Ok(hour * 60 + min)
}

fn time_range(str_time1: &str, str_time2: &str) -> Result<u32, Error> {
    let t1 = parse_time(str_time1)?;
    let t2 = parse_time(str_time2)?;

    if t2 < t1 {
        let error_msg = format!("Horário {} é maior que horário {}", str_time1, str_time2);
        return Err(Error::NotOrdered(error_msg));
    }
    
    Ok(t2 - t1)
}

pub fn run() -> Result<f32, Error> {
    let inp = input::parse()?;

    if let input::Input::DoubleInterval(i1, i2) = inp {

        let h1 = &i1.begin;
        let h2 = &i1.end;
        let h3 = &i2.begin;
        let h4 = &i2.end;

        let delta_t1 = time_range(h1, h2)?;
        let delta_t2 = time_range(h3, h4)?;

        let total = (delta_t1 + delta_t2) as f32 / 60.0;
        return Ok(total);
    } else {
        unimplemented!("Implementar outras formas de entrada de dados...");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_time_should_work() {
        assert_eq!(parse_time("00:00"), Ok(0u32));
        assert_eq!(parse_time("01:30"), Ok(90u32));
    }

    #[test]
    fn parse_time_should_fail_with_parsing_error() {
        let assert_parse_time_parsing_error = |res: Result<u32, Error>| {
            match res {
                Ok(x) => panic!("Wrong format must fail parsing with ParsingError. Got Ok({})", x),
                Err(e) =>
                    assert_eq!(
                        std::mem::discriminant(&e),
                        std::mem::discriminant(&Error::ParsingError("".to_string()))
                    )
            }
        };

        assert_parse_time_parsing_error(parse_time("1A:23"));
        assert_parse_time_parsing_error(parse_time("1234"));
        assert_parse_time_parsing_error(parse_time("12:345"));
        assert_parse_time_parsing_error(parse_time("01:75"));
        assert_parse_time_parsing_error(parse_time("24:30"));
    }
}
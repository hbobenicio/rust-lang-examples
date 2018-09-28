use std::env;
use std::num;
use std::fmt;

enum AppError {
    WrongNumberOfArgs(usize),
    ParsingError(String),
    NotOrdered(String)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::WrongNumberOfArgs(n) =>
                write!(f, "Número errado de argumentos. Necessário 4, passados apenas {}", n),
            AppError::ParsingError(s) =>
                write!(f, "Falha no parsing: {}", s),
            AppError::NotOrdered(s) =>
                write!(f, "Horários não estão em ordem crescente: {}", s)
        }
    }
}

type AppResult = Result<f32, AppError>;

impl std::convert::From<std::string::ParseError> for AppError {
    fn from(other: std::string::ParseError) -> Self {
        AppError::ParsingError(other.to_string())
    }
}

impl std::convert::From<num::ParseIntError> for AppError {
    fn from(other: num::ParseIntError) -> Self {
        AppError::ParsingError(other.to_string())
    }
}

fn parse_time(str_time: &str) -> Result<u32, AppError> {
    let len: usize = str_time.chars().count();
    let has_separator: bool = str_time.chars().nth(2) == Some(':');

    if len != 5 || !has_separator {
        let error_msg = format!("Horário {} não é um formato de hora válida (hh:mm)", str_time);
        return Err(AppError::ParsingError(error_msg));
    }

    let parts: Vec<&str> = str_time.split(':').collect();
    let hour: u32 = parts[0].parse()?;
    let min: u32 = parts[1].parse()?;

    Ok(hour * 60 + min)
}

fn time_range(str_time1: &str, str_time2: &str) -> Result<u32, AppError> {
    let t1 = parse_time(str_time1)?;
    let t2 = parse_time(str_time2)?;

    if t2 < t1 {
        let error_msg = format!("Horário {} é maior que horário {}", str_time1, str_time2);
        return Err(AppError::NotOrdered(error_msg));
    }
    
    Ok(t2 - t1)
}

fn start() -> AppResult {
    let args: Vec<String> = env::args()
        .skip(1)
        .collect();

    if args.len() != 4 {
        return Err(AppError::WrongNumberOfArgs(args.len()));
    }

    let h1 = &args[0];
    let h2 = &args[1];
    let h3 = &args[2];
    let h4 = &args[3];

    let delta_t1 = time_range(h1, h2)?;
    let delta_t2 = time_range(h3, h4)?;

    let total = (delta_t1 + delta_t2) as f32 / 60.0;
    Ok(total)
}

fn main() {
    match start() {
        Ok(total) => println!("Horas trabalhadas: {}h", total),
        Err(reason) => eprintln!("Error: {}", reason)
    };
}

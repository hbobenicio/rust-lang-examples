use crate::app;

pub struct Interval {
    pub begin: String,
    pub end: String,
}

pub enum Input {
    SingleInterval(Interval),
    DoubleInterval(Interval, Interval),
    File {path: String},
}

/// Detect detecta qual o tipo de entrada, com base nos argumentos da linha de comando
pub fn parse() -> Result<Input, app::Error> {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .collect();

    let nargs = args.len();

    match nargs {
        1 => Ok(parse_file_input(&args)),
        2 => Ok(parse_single_interval_input(&args)),
        4 => Ok(parse_double_interval_input(&args)),
        _ => Err(app::Error::WrongNumberOfArgs(nargs)),
    }
}

fn parse_file_input(args: &Vec<String>) -> Input {
    Input::File {
        path: args[0].clone(),
    }
}

fn parse_single_interval_input(args: &Vec<String>) -> Input {
    let (begin, end) = (args[0].clone(), args[1].clone());

    Input::SingleInterval(Interval{begin, end})
}

fn parse_double_interval_input(args: &Vec<String>) -> Input {
    let i1 = Interval {
        begin: args[0].clone(),
        end: args[1].clone()
    };

    let i2 = Interval {
        begin: args[2].clone(),
        end: args[3].clone(),
    };

    Input::DoubleInterval(i1, i2)
}

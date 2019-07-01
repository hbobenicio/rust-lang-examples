/// Módule responsible for parsings (times and file records)

use app::error::Error;

/// Registro do arquivo de entrada, contendo todos os horários
#[derive(Debug, Eq, PartialEq)]
pub struct FileRecord {
    pub day: String,
    pub times: Vec<String>,
}

/// Realiza o parsing horário (`"08:30"`) para minutos (`8 * 60 + 30 = 510u32`)
pub fn parse_time(str_time: &str) -> Result<u32, Error> {
    let len: usize = str_time.chars().count();
    let has_separator: bool = str_time.chars().nth(2) == Some(':');

    if len != 5 || !has_separator {
        let error_msg = format!("horário '{}' não é um formato de hora válida (hh:mm)", str_time);
        return Err(Error::ParsingError(error_msg));
    }

    let parts: Vec<&str> = str_time.split(':').collect();
    let hour: u32 = parts[0].parse()?;
    let min: u32 = parts[1].parse()?;

    if min >= 60 {
        let error_msg = format!("minutos devem ser menores que 60");
        return Err(Error::ParsingError(error_msg));
    }

    if hour >= 24 {
        let error_msg = format!("horas trabalhadas devem ser menores que 24");
        return Err(Error::ParsingError(error_msg));
    }

    Ok(hour * 60 + min)
}

/// Realiza o parsing na linha (registro) do arquivo de entrada com os horários
/// 
/// # Argumentos
/// 
/// * `line` - Linha (Registro)
/// 
/// # Return
/// 
/// Talvez um registro. Caso a linha esteja bem formada (layout ok), mas
/// não possui nenhum horário (por exemplo, final de semana), retorna `Ok(None)`
pub fn parse_line(line: &str) -> Result<Option<FileRecord>, Error> {

    // Filtragem básica inicial (remove espaços em geral)
    let fields: Vec<&str> = line.trim().split_whitespace()
        .map(|f| f.trim())
        .filter(|f| !f.is_empty())
        .collect();

    // Obtém o dia (primeiro campo antes do primeiro horário).
    // Caso não existam horários (final de semana, por exemplo),
    // day receberá None
    let mut day: Option<String> = None;
    let mut peekable_iter = fields.iter().peekable();
    while let Some(field) = peekable_iter.next() {
        if let Some(v) = peekable_iter.peek() {
            if v.contains(":") {
                day = Some(field.to_string());
                break;
            }
        }
    }

    // Caso não encontre-mos nenhum horário, não há problema,
    // pois pode se tratar de um final de semana.
    // Logo, não retornamos erro, mas não existem valores também.
    let day = match day {
        Some(d) => d,
        None => {
            return Ok(None);
        }
    };

    // Filtra os campos do registro em busca apenas dos horários,
    // identificados pelo caracter ':'
    let times: Vec<String> = fields.iter()
        .filter(|f| f.contains(":"))
        .map(|f| f.to_string())
        .collect();

    Ok(Some(FileRecord{day, times}))
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

    #[test]
    fn parse_line_should_work() {
        struct TestCase {
            pub input: String,
            pub expected: Result<Option<FileRecord>, Error>,
        }

        let test_cases = [
            TestCase {
                input: "84              22      08:15   11:51   13:03   17:18                                                                   -9      0".to_string(),
                expected: Ok(Some(FileRecord {
                    day: "22".to_string(),
                    times: vec!(
                        "08:15".to_string(),
                        "11:51".to_string(),
                        "13:03".to_string(),
                        "17:18".to_string()
                    )
                })),
            },
            TestCase {
                input: "                29      07:46   12:25   13:37   17:25                                                                   27      27".to_string(),
                expected: Ok(Some(FileRecord {
                    day: "29".to_string(),
                    times: vec!(
                        "07:46".to_string(),
                        "12:25".to_string(),
                        "13:37".to_string(),
                        "17:25".to_string()
                    )
                })),
            },
            TestCase {
                input: "                31                                                                                                      0       0".to_string(),
                expected: Ok(None),
            },
            TestCase {
                input: "84		4	07:53	12:41	12:50	12:51	13:56	16:55							-12	0".to_string(),
                expected: Ok(Some(FileRecord {
                    day: "4".to_string(),
                    times: vec!(
                        "07:53".to_string(),
                        "12:41".to_string(),
                        "12:50".to_string(),
                        "12:51".to_string(),
                        "13:56".to_string(),
                        "16:55".to_string(),
                    )
                })),
            }
        ];

        for test_case in &test_cases {
            let actual = parse_line(&test_case.input);
            assert_eq!(actual, test_case.expected);
        }
    }
}

mod error;
mod input;
mod parser;

use std::io::BufRead;
use app::error::Error;

use prettytable::{row, cell};

/// Inicia a execução da aplicação
pub fn run() -> Result<f32, Error> {
    let inp = input::parse()?;

    // Dependendo do tipo de entrada especificada pela linha de comando,
    // Realiza a operação correta
    match inp {
        input::Input::DoubleInterval(i1, i2) => {
            let h1 = &i1.begin;
            let h2 = &i1.end;
            let h3 = &i2.begin;
            let h4 = &i2.end;

            let delta_t1 = time_range(h1, h2)?;
            let delta_t2 = time_range(h3, h4)?;

            let total = (delta_t1 + delta_t2) as f32 / 60.0;
            Ok(total)
        },
        input::Input::SingleInterval(i) => {
            let h1 = &i.begin;
            let h2 = &i.end;

            let delta_t = time_range(h1, h2)?;
            let delta_t = delta_t as f32 / 60.0;

            Ok(delta_t as f32)
        },
        input::Input::File {path} => {
            let total_horas_trabalhadas = process_file(path)?;

            Ok(total_horas_trabalhadas)
        }
    }
}

fn time_range(str_time1: &str, str_time2: &str) -> Result<u32, Error> {
    let t1 = parser::parse_time(str_time1)?;
    let t2 = parser::parse_time(str_time2)?;

    if t2 < t1 {
        let error_msg = format!("Horário {} é maior que horário {}", str_time1, str_time2);
        return Err(Error::NotOrdered(error_msg));
    }
    
    Ok(t2 - t1)
}

fn process_file(path: String) -> Result<f32, Error> {
    let mut table = prettytable::Table::new();
    table.set_titles(row![bFg => "DIA", "HORÁRIOS", "HORAS TRABALHADAS", "APROPRIAÇÃO"]);

    let mut total_horas_trabalhadas: f32 = 0.0;
    
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        // Scans the BufRead for a line and converts it to a Rust String (which is UTF-8)
        let line: String = line?;

        // Parses the line and tries to get the day and related times only
        let record = match parser::parse_line(&line)? {
            Some(r) => r,
            None => continue,
        };

        // TODO Melhorar este bloco de código (Match e/ou Loop)
        if record.times.len() == 2 {
            let h1 = &record.times[0];
            let h2 = &record.times[1];

            let delta_t = time_range(h1, h2)?;
            let delta_t = delta_t as f32 / 60.0;
            total_horas_trabalhadas += delta_t;

            table.add_row(row![
                bFw -> record.day,
                format!("{} às {}", h1, h2),
                delta_t,
                bFw -> format!("{:.1}", delta_t - 0.1)
            ]);
            
        } else if record.times.len() == 4 {
            let h1 = &record.times[0];
            let h2 = &record.times[1];
            let h3 = &record.times[2];
            let h4 = &record.times[3];

            let delta_t1 = time_range(h1, h2)?;
            let delta_t2 = time_range(h3, h4)?;

            let total = (delta_t1 + delta_t2) as f32 / 60.0;
            total_horas_trabalhadas += total;

            table.add_row(row![
                bFw -> record.day,
                format!("{} às {}, {} às {}", h1, h2, h3, h4),
                total,
                bFw -> format!("{:.1}", total - 0.1)
            ]);
        } else if record.times.len() == 6 {
            let h1 = &record.times[0];
            let h2 = &record.times[1];
            let h3 = &record.times[2];
            let h4 = &record.times[3];
            let h5 = &record.times[4];
            let h6 = &record.times[5];

            let delta_t1 = time_range(h1, h2)?;
            let delta_t2 = time_range(h3, h4)?;
            let delta_t3 = time_range(h5, h6)?;

            let total = (delta_t1 + delta_t2 + delta_t3) as f32 / 60.0;
            total_horas_trabalhadas += total;

            table.add_row(row![
                bFw -> record.day,
                format!("{} às {}, {} às {} e {} às {}", h1, h2, h3, h4, h5, h6),
                total,
                bFw -> format!("{:.1}", total - 0.1)
            ]);
        } else {
            eprintln!("dia {}: ignorado! {:?}", record.day, record);
        }
    }

    table.printstd();

    Ok(total_horas_trabalhadas)
}

#[cfg(test)]
mod test {
    // use super::*;

    
}

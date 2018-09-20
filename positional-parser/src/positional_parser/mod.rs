pub mod error;
mod model;

use std::result;
use std::io;
use std::io::BufRead;
use std::collections;
use std::fs::File;
use serde_yaml;
use self::error::Error;
use self::model::MetaInfo;

pub type Result<T> = result::Result<T, self::error::Error>;

fn open_file(filepath: &str) -> Result<File> {
    File::open(filepath).map_err(|e: io::Error|
        Error {
          code: String::from("ER001"),
          message: format!("Erro ao abrir arquivo \"{}\": {}", filepath, e)
        }
    )
}

fn parse<R: io::Read>(file_reader: R) -> Result<MetaInfo> {
    serde_yaml::from_reader(file_reader).map_err(|e: serde_yaml::Error|
        Error {
          code: String::from("ER002"),
          message: format!("Erro ao fazer parsing do schema: {}", e)
        }
    )
}

pub fn start(schema_filepath: &str, data_filepath: &str) -> Result<()> {
    let schema_file = open_file(schema_filepath)?;
    let meta_info = parse(schema_file)?;

    let data_file: File = open_file(data_filepath)?;

    // File implements Read, but not BufRead
    let buffered_data_file = io::BufReader::new(data_file);
    
    let mut data = vec![];
    for line in buffered_data_file.lines().map(|l| l.unwrap()) {
      let mut record = collections::HashMap::new();
      let mut offset = 0;
      // for (key, value) in meta_info.fields.iter() {
      for field in &meta_info.fields {
        for (key, value) in field {
          let size = value.size;
          let field = String::from(&line[offset..offset + size]);
          record.insert(key, field);
          offset += size;
        }
      }
      data.push(record);
    }

    println!("{:?}", data);

    Ok(())
}

use chrono::Local;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, Write};
use std::path::Path;

/*
* Very rough logger
*/

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    INFO,
    DEBUG,
    WARNING,
    ERROR,
}

fn inspect(level: LogLevel) -> String {
    match level {
        LogLevel::INFO => "INFO".to_owned(),
        LogLevel::DEBUG => "DEBUG".to_owned(),
        LogLevel::WARNING => "WARNING".to_owned(),
        LogLevel::ERROR => "ERROR".to_owned(),
    }
}

#[derive(Clone, Debug)]
pub struct Logger {
    level: LogLevel,
    // file: String,
    description: String,
    // time: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            level: LogLevel::DEBUG,
            // file: Path::new(file!())
            //     .file_name()
            //     .and_then(|s| s.to_str())
            //     .unwrap()
            //     .to_string(),
            description: "".to_owned(),
            // time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn set_level(&mut self, l: LogLevel) {
        self.level = l
    }

    pub fn set_description(&mut self, s: &str) {
        self.description = s.to_string();
    }

    pub fn write_to_file(&self, filename: &str) -> std::io::Result<()> {
        let file = OpenOptions::new().append(true).create(true).open(filename);
        file.expect("an error has occurred while working on the file")
            .write_all(self.to_string().as_bytes())?;
        Ok(())
    }
}

// ridefinizione operatore di stampa
impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] - {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            inspect(self.level),
            self.description
        )
    }
}

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

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
    path: String,
}

impl Logger {
    pub fn new(path: &str) -> Self {
        Logger {
            level: LogLevel::DEBUG,
            path: path.to_owned(),
        }
    }

    pub fn set_level(&mut self, l: LogLevel) {
        self.level = l
    }

    pub fn write(&self, description: &str) -> std::io::Result<()> {
        let content = format!(
            "{} [{}] - {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            inspect(self.level),
            description
        );
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.path.to_string());
        file.expect("an error has occurred while working on the file")
            .write_all(content.as_bytes())?;
        Ok(())
    }
}

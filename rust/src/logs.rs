use std::{fs::{File, OpenOptions}, error::Error, fmt};
use crate::user::{User, UserStatus};

use csv::{Writer, WriterBuilder};
use serde::Serialize;
use chrono::{DateTime, Local};

#[derive(Serialize)]
pub struct Log {
    pub time: DateTime<Local>,
    pub uid: String,
    pub surname: String,
    pub name: String,
    pub status: UserStatus
}

impl Log {

    pub fn from(user: User) -> Self {
        Self {
            time: Local::now(),
            uid: user.uid,
            surname: user.surname,
            name: user.name,
            status: user.status
        }
    }

}

#[derive(Debug)]
pub enum LogsError {
    LoadingError,
    SaveError
}

impl fmt::Display for LogsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl Error for LogsError {}

pub struct Logger {
    writer: Writer<File>
}

impl Logger {
    
    pub fn new(path: &str) -> Result<Self, LogsError> {
        Ok(Self { 
            writer: Logger::create_writer(path)?
        })
    }

    fn create_writer(path: &str) -> Result<Writer<File>, LogsError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|_| LogsError::LoadingError)?;

        let writer = WriterBuilder::new().has_headers(false).from_writer(file);
        return Ok(writer);
    }

    pub fn add(&mut self, user: User) -> Result<(), LogsError> {
        let log = Log::from(user);

        self.writer.serialize(log).map_err(|_| LogsError::SaveError)?;
        self.writer.flush().map_err(|_| LogsError::SaveError)?;
        Ok(())
    }
}
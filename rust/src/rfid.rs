use serialport::{SerialPortInfo, SerialPort};

use std::time::Duration;
use std::io::{BufReader, BufRead};
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RFIDError {
    ReaderNotFound,
    ReadFailed
}

impl fmt::Display for RFIDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl Error for RFIDError {}


// CARD READER

pub struct RFIDReader {
    // port: Box<dyn SerialPort>,
    reader: BufReader<Box<dyn SerialPort>>
}

impl RFIDReader {

    pub fn new() -> Result<Self, RFIDError> {
        let port_name = RFIDReader::find_reader()?;
        let port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(1000))
            .open()
            .map_err(|_| RFIDError::ReaderNotFound)?;
        
        Ok(Self { reader: BufReader::new(port) })
    }

    pub fn wait_until_read(&mut self) -> Result<String, RFIDError> {
        let mut output = String::new();
        
        while output == "" {
            let _ = self.reader.read_line(&mut output).map_err(|_| RFIDError::ReadFailed);
        }
        output = String::from(output.trim());

        Ok(output)
    }

    fn find_reader() -> Result<String, RFIDError> {
        let ports = serialport::available_ports().map_err(|_| RFIDError::ReaderNotFound)?;
        let ports: Vec<SerialPortInfo> = ports.into_iter().filter(|x| x.port_name.contains("ACM")).collect();
    
        if ports.len() != 1 {
            return Err(RFIDError::ReaderNotFound);
        }
    
        let port = ports.get(0).unwrap().port_name.clone();
        Ok(port)
    }

}
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::fs;

#[derive(Debug)]
pub struct AppError {
    description: String,
    cause: Option<Box<Error>>,
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        match self.cause {
            Some(ref e) => Some(&**e),
            None => None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        fmt.write_str(self.description())
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            description: error.description().into(),
            cause: Some(Box::new(error)),
        }
    }
}

fn io_fail() -> Result<(), AppError> {
    let _f = try!(fs::File::open("nonexistent"));
    Ok(())
}

fn main() {
    println!("error = {}", io_fail().unwrap_err());
}

#[derive(Debug)]
pub enum Error {
    Error(String),
    DBError(houseflow_db::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Error(err.to_string())
    }
}

impl From<houseflow_db::Error> for Error {
    fn from(err: houseflow_db::Error) -> Error {
        Error::DBError(err)
    }
}

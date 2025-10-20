#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Forbidden(String),
    OperationNotApplicable(String),
    Unknown(String),
}

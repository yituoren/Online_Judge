use serde::Serialize;

//my httperror type
#[derive(Debug, Serialize)]
pub struct HttpError
{
    pub code: u32,
    pub reason: String,
    pub message: String,
}
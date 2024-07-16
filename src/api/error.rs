use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HttpError
{
    pub code: u32,
    pub reason: String,
    pub message: String,
}
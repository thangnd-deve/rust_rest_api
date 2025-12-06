use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T> {
    pub code: u16,
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}
use alloc::string::String;

#[derive(Default)]
pub struct Request {
    pub message: String,
}

#[derive(Default)]
pub struct Response {
    pub message: String,
}

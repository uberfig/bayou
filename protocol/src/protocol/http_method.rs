#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
}
impl HttpMethod {
    pub fn stringify(&self) -> &str {
        match self {
            HttpMethod::Get => "get",
            HttpMethod::Post => "post",
        }
    }
}

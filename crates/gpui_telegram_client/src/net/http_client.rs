pub struct HttpClient {
    pub base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

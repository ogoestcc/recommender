#[derive(Debug, serde::Deserialize)]
pub struct ServerConfig {
    pub port: u32,
}

impl ServerConfig {
    pub fn get_url(&self) -> String {
        format!("[::1]:{}", self.port)
    }
}
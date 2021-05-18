fn default_protocol() -> String {
    "http".into()
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_protocol")]
    pub protocol: String,
}

impl DatabaseConfig {
    pub fn get_connection_url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

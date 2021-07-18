fn default_protocol() -> String {
    "http".into()
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
struct Credential {
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn get_credential(&self) -> String {
        format!("{}:{}@", self.username, self.password)
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: Option<u16>,
    pub database: Option<String>,
    credentials: Option<Credential>,
}

impl RedisConfig {
    pub fn get_connection_url(&self) -> String {
        let mut protocol = "redis://".to_owned();

        if let Some(cred) = &self.credentials {
            protocol += &cred.get_credential();
        }

        protocol += &self.host;

        if let Some(port) = &self.port {
            protocol += &format!(":{}", port);
        }

        if let Some(database) = &self.database {
            protocol += &format!("/{}", database);
        }

        protocol
    }
}

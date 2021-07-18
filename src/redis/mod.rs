mod alerts;
mod users;

use crate::config::redis::RedisConfig;

#[derive(Clone)]
pub struct Redis {
    config: RedisConfig,
    client: redis::Client,
}

impl Redis {
    pub fn new(config: RedisConfig) -> Self {
        Self {
            client: redis::Client::open(config.get_connection_url()).unwrap(),
            config,
        }
    }

    pub fn zip_key_value<V: Clone>(keys_and_values: Vec<V>) -> Vec<(V, V)> {
        let mut part = false;
        let (keys, values): (Vec<_>, Vec<_>) = keys_and_values.iter().partition(|_| {
            part = !part;
            part
        });

        keys.iter()
            .zip(values.iter())
            .map(|(&v, &va)| (v.to_owned(), va.to_owned()))
            .collect()
    }
}

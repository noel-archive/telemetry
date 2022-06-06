// 🐻‍❄️🌧️ Noelware Telemetry: Telemetry project for Noelware to capture anonymous data about our running products.
// Copyright 2022 Noelware <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::{env::var, fs::read_to_string};

static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub clickhouse: Option<ClickHouseConfig>, // defaults to { host: "localhost", port: 9000, database: "telemetry" }
    pub sentry_dsn: Option<String>,
    pub logging: Option<LogConfig>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    pub json: Option<bool>,
    pub level: Option<String>,
    pub logstash_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickHouseConfig {
    pub min_connections_in_pool: Option<u16>, // defaults to 10
    pub max_connections_in_pool: Option<u16>, // defaults to 20
    pub use_lz4_compression: Option<bool>,    // defaults to "false"
    pub database: Option<String>,             // defaults to "telemetry"
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl ToString for ClickHouseConfig {
    fn to_string(&self) -> String {
        // is this a bad idea? probably.
        // if you think this is bad, smash star on this repository :)
        let mut url = String::from("tcp://");

        if self.username.is_some() && self.password.is_some() {
            let pass = self.password.as_ref().unwrap();
            let user = self.username.as_ref().unwrap();

            url.push_str(user.as_str());
            url.push(':');
            url.push_str(pass.as_str());
            url.push('@');
        }

        url.push_str(
            self.host
                .as_ref()
                .unwrap_or(&"localhost".to_string())
                .as_str(),
        );

        url.push(':');
        url.push_str(self.port.as_ref().unwrap_or(&9000).to_string().as_str());

        // now we're at the point of:
        // tcp://<username>:<password>@<host>:<port>
        //
        // now we need to append db name and parameters. this is fine.
        url.push('/');
        url.push_str(
            self.database
                .as_ref()
                .unwrap_or(&"telemetry".to_string())
                .as_str(),
        );

        let mut prefix = '?';
        match self.use_lz4_compression {
            Some(b) if b => {
                url.push_str("?compression=lz4");
                if prefix == '?' {
                    prefix = '&';
                }
            }

            Some(_) => {} // do nothing
            None => {}    // do nothing
        }

        match self.max_connections_in_pool {
            Some(max_conn) => {
                url.push(prefix);
                let _ = write!(url, "pool_max={}", max_conn);

                if prefix == '?' {
                    prefix = '&';
                }
            }

            None => {}
        }

        match self.min_connections_in_pool {
            Some(min_conn) => {
                url.push(prefix);
                let _ = write!(url, "pool_max={}", min_conn);
            }

            None => {}
        }

        url
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn url_tests_without_params_and_auth() {
        let config = crate::config::ClickHouseConfig {
            min_connections_in_pool: None,
            max_connections_in_pool: None,
            use_lz4_compression: None,
            database: Some("telemetry".into()),
            username: None,
            password: None,
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url = config.to_string();
        assert_eq!(url, "tcp://localhost:9000/telemetry");
    }

    #[test]
    fn url_tests_with_auth_but_not_params() {
        let config = crate::config::ClickHouseConfig {
            min_connections_in_pool: None,
            max_connections_in_pool: None,
            use_lz4_compression: None,
            database: Some("telemetry".into()),
            username: Some("noel".into()),
            password: Some("noelisthebest".into()),
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url = config.to_string();
        assert_eq!(url, "tcp://noel:noelisthebest@localhost:9000/telemetry");
    }

    #[test]
    fn url_tests_with_params_and_auth() {
        let config = crate::config::ClickHouseConfig {
            min_connections_in_pool: Some(10),
            max_connections_in_pool: Some(300),
            use_lz4_compression: Some(true),
            database: Some("telemetry".into()),
            username: Some("noel".into()),
            password: Some("noelisthebest".into()),
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url = config.to_string();
        assert_eq!(url, "tcp://noel:noelisthebest@localhost:9000/telemetry?compression=lz4&pool_max=300&pool_min=10");

        let config_2 = crate::config::ClickHouseConfig {
            min_connections_in_pool: Some(69),
            max_connections_in_pool: Some(420),
            use_lz4_compression: None,
            database: Some("telemetry".into()),
            username: Some("noel".into()),
            password: Some("noelisthebest".into()),
            host: Some("localhost".into()),
            port: Some(9000),
        };

        let url2 = config_2.to_string();
        assert_eq!(
            url2,
            "tcp://noel:noelisthebest@localhost:9000/telemetry?pool_max=420&pool_min=69"
        );
    }
}

impl Config {
    /// Allows to load the configuration from the system environment variables. All
    /// environment variables must be prefixed with `TELEMETRY_` to be usuable.
    ///
    /// ## Options
    /// | Config Key                                    | Environment Variable Name              | Required | Type        |
    /// | :-------------------------------------------- | :------------------------------------- | :-------- | :--------- |
    /// | `config.sentry_dsn`                          | TELEMETRY_SENTRY_DSN                    | false    | **String** |
    /// | `config.logging.level`                       | TELEMETRY_LOG_LEVEL                     | false    | **String** |
    /// | `config.logging.logstash_uri`               | TELEMETRY_LOGSTASH_URI                   | false    | **URI**    |
    /// | `config.clickhouse.min_connections_in_pool` | TELEMETRY_CLICKHOUSE_MIN_CONN_IN_POOL    | false    | **u16**    |
    /// | `config.clickhouse.max_connections_in_pool` | TELEMETRY_CLICKHOUSE_MAX_CONN_IN_POOL    | false    | **u16**    |
    /// | `config.clickhouse.use_lz4_compression`     | TELEMETRY_CLICKHOUSE_USE_LZ4_COMPRESSION | false    | **Bool**   |
    /// | `config.clickhouse.database`                | TELEMETRY_CLICKHOUSE_DB_NAME            | false     | **String** |
    /// | `config.clickhouse.username`                | TELEMETRY_CLICKHOUSE_USERNAME           | false     | **String** |
    /// | `config.clickhouse.password`                | TELEMETRY_CLICKHOUSE_PASSWORD           | false     | **String** |
    /// | `config.clickhouse.host`                    | TELEMETRY_CLICKHOUSE_HOST               | false     | **String** |
    /// | `config.clickhouse.port`                    | TELEMETRY_CLICKHOUSE_PORT               | false     | **u16**    |
    /// | `config.host`                               | TELEMETRY_HTTP_HOST                     | false     | **String** |
    /// | `config.port`                               | TELEMETRY_HTTP_PORT                     | false     | **u16**    |
    fn from_env() -> Config {
        let sentry_dsn = var("TELEMETRY_SENTRY_DSN").ok();
        let log_level = var("TELEMETRY_LOG_LEVEL").ok();
        let logstash_endpoint = var("TELEMETRY_LOGSTASH_URI").ok();
        let log_in_json = var("TELEMETRY_LOG_IN_JSON").ok();
        let min_conn_in_pool = var("TELEMETRY_CLICKHOUSE_MIN_CONN_IN_POOL").ok();
        let max_conn_in_pool = var("TELEMETRY_CLICKHOUSE_MAX_CONN_IN_POOL").ok();
        let use_lz4_compression = var("TELEMETRY_CLICKHOUSE_USE_LZ4_COMPRESSION").ok();
        let clickhouse_database = var("TELEMETRY_CLICKHOUSE_DB_NAME").ok();
        let clickhouse_username = var("TELEMETRY_CLICKHOUSE_USERNAME").ok();
        let clickhouse_password = var("TELEMETRY_CLICKHOUSE_PASSWORD").ok();
        let clickhouse_host = var("TELEMETRY_CLICKHOUSE_HOST").ok();
        let clickhouse_port = var("TELEMETRY_CLICKHOUSE_PORT").ok();
        let host = var("TELEMETRY_HTTP_HOST").ok();
        let port = var("TELEMETRY_HTTP_PORT").ok();

        let clickhouse_config = ClickHouseConfig {
            min_connections_in_pool: min_conn_in_pool
                .map(|p| p.parse::<u16>().expect("Unable to convert String -> u16")),

            max_connections_in_pool: max_conn_in_pool
                .map(|p| p.parse::<u16>().expect("Unable to convert String -> u16")),

            use_lz4_compression: use_lz4_compression
                .map(|p| p.parse::<bool>().expect("Unable to convert String -> bool")),

            database: clickhouse_database,
            username: clickhouse_username,
            password: clickhouse_password,
            host: clickhouse_host,
            port: clickhouse_port
                .map(|p| p.parse::<u16>().expect("Unable to convert String -> u16")),
        };

        Config {
            clickhouse: Some(clickhouse_config),
            sentry_dsn,
            logging: Some(LogConfig {
                level: log_level,
                logstash_uri: logstash_endpoint,
                json: log_in_json
                    .map(|p| p.parse::<bool>().expect("Unable to convert String -> bool")),
            }),

            host,
            port: port.map(|p| p.parse::<u16>().expect("Unable to convert String -> u16")),
        }
    }

    pub fn load() {
        let path = if let Ok(path) = var("TELEMETRY_CONFIG_PATH") {
            path
        } else {
            "config.toml".into()
        };

        let contents = read_to_string(path);
        let cfg = match contents {
            Ok(contents) => {
                toml::from_str::<Config>(&contents).expect("Unable to parse 'config.toml' contents")
            }
            Err(_) => Config::from_env(),
        };

        CONFIG.set(cfg).expect("Unable to set configuration cell.");
    }

    pub fn get() -> &'static Config {
        CONFIG.get().unwrap()
    }
}

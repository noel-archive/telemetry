// 🐻‍❄️🌧️ Noelware Telemetry: Telemetry project for Noelware, to capture anonymous data about the running products.
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

use std::net::SocketAddr;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};

use crate::{clickhouse::ClickHouse, config::Config, routes};

#[derive(Debug, Clone)]
pub struct TelemetryServer {
    pub config: &'static Config,
    pub clickhouse: ClickHouse,
}

impl TelemetryServer {
    pub fn new(clickhouse: ClickHouse) -> TelemetryServer {
        TelemetryServer {
            config: Config::get(),
            clickhouse,
        }
    }

    pub async fn launch(self) -> Result<(), Box<dyn std::error::Error>> {
        info!("checking if clickhouse conn is safe");

        let clickhouse = self.clickhouse.clone();
        let result = clickhouse.ping().await;
        if let Err(error) = result {
            error!("{}", error);
            panic!("Couldn't ping ClickHouse, view above error on why.");
        }

        info!("launching http service...");
        let addr = match &self.config.host {
            Some(host) => {
                let port = self.config.port.unwrap_or(1234);
                format!("{}:{}", host, port)
                    .parse::<SocketAddr>()
                    .expect("Unable to parse to SocketAddr.")
            }
            None => {
                let port = self.config.port.unwrap_or(1234);
                format!("0.0.0.0:{}", port)
                    .parse::<SocketAddr>()
                    .expect("Unable to parse to SocketAddr.")
            }
        };

        info!("now running in address {addr}!");
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(self.clone()))
                .wrap(Logger::new("%r %s [%b bytes; %D ms]").log_target("actix::http::request"))
                .route("/", web::get().to(routes::home))
                .route("/stats", web::get().to(routes::stats))
        })
        .bind(addr)?
        .run()
        .await?;

        Ok(())
    }
}

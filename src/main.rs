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

use config::Config;

use crate::{clickhouse::ClickHouse, telemetry::TelemetryServer};

#[macro_use]
extern crate log;
extern crate actix_web;
extern crate futures;

mod clickhouse;
mod config;
mod constants;
mod responses;
mod routes;
mod setup_utils;
mod snowflake;
mod telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::load();

    let config = Config::get();
    setup_utils::setup_logging(config)?;
    setup_utils::setup_sentry(config)?;

    info!(
        "bootstrapping v{} ({}) of telemetry-server...",
        constants::VERSION,
        constants::COMMIT_HASH
    );

    let clickhouse = ClickHouse::new(config.clickhouse.as_ref().unwrap());
    let server = TelemetryServer::new(clickhouse.clone());
    server.launch().await?;

    Ok(())
}

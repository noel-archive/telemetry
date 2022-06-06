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

use std::sync::atomic::{AtomicUsize, Ordering};

use clickhouse_rs::{types::Complex, Block, Pool};
use once_cell::sync::OnceCell;

use crate::config::ClickHouseConfig;

static DATABASE_CALLS: OnceCell<AtomicUsize> = OnceCell::new();

/// Represents the main ClickHouse connection with methods to query
/// objects with a simple `.sql("<query>", move |result| {})` function.
#[derive(Debug, Clone)]
pub struct ClickHouse {
    #[allow(dead_code)]
    pool: Pool,
}

impl ClickHouse {
    pub fn new(config: &ClickHouseConfig) -> ClickHouse {
        let url = config.to_string();
        let pool = Pool::new(url);

        DATABASE_CALLS
            .set(AtomicUsize::new(0))
            .expect("Unable to create db calls constant.");

        ClickHouse { pool }
    }

    pub async fn ping(self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("grabbing connection...");
        let pool = self.pool.clone();
        let mut handle = pool.get_handle().await?;

        DATABASE_CALLS.get().unwrap().fetch_add(1, Ordering::SeqCst);
        handle.ping().await?;

        Ok(())
    }

    pub async fn query<S, F, U>(&self, sql: S, func: F) -> Result<U, Box<dyn std::error::Error>>
    where
        S: Into<String> + AsRef<str>,
        F: Fn(Block<Complex>) -> U,
    {
        debug!("grabbing connection...");
        let pool = self.pool.clone();
        let mut handle = pool.get_handle().await?;

        debug!("grabbed connection successfully!");
        DATABASE_CALLS.get().unwrap().fetch_add(1, Ordering::SeqCst);

        let block = handle.query(sql).fetch_all().await?;
        let result = func(block);

        Ok(result)
    }
}

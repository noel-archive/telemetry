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

use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc, Mutex,
    },
    time::SystemTime,
};

use chrono::{DateTime, Utc};

/// Represents a Twitter snowflake for Noelware's use. The epoch is June 1st, 2022 at 00:00 UTC.
#[derive(Clone, Debug)]
pub struct Snowflake {
    increment: Arc<Mutex<AtomicI64>>,
}

impl Snowflake {
    pub fn new() -> Snowflake {
        Snowflake {
            increment: Arc::new(Mutex::new(AtomicI64::new(0))),
        }
    }

    pub fn generate(&mut self) -> i64 {
        let now = SystemTime::now();
        let time: DateTime<Utc> = now.into();
        let timestamp = time.timestamp_millis();

        let mut increment = self.increment.try_lock().unwrap();
        let old_increment = increment.fetch_add(1, Ordering::SeqCst);

        if old_increment + 1 > 4095 {
            let i = increment.get_mut();
            *i = 0;
        }

        increment.store(old_increment + 1, Ordering::SeqCst);
        let curr = increment.load(Ordering::SeqCst);

        ((timestamp - 1654066800000) << 22) | (0b11111 << 17) | ((1 & 0b11111) << 12) | curr
    }
}

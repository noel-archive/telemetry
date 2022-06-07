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

use std::env;

/// Returns the current version of `telemetry-server`.
#[allow(dead_code)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the commit hash from the [Noelware/telemetry repository](https://github.com/Noelware/telemetry repository)
#[allow(dead_code)]
pub const COMMIT_HASH: &str = env!("TELEMETRY_COMMIT_HASH");

/// Returns the build date of when the Telemetry server was last built.
#[allow(dead_code)]
pub const BUILD_DATE: &str = env!("TELEMETRY_BUILD_DATE");

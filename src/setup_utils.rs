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

use std::{
    env::var,
    io::Write,
    net::{SocketAddr, TcpStream},
    str::FromStr,
};

use ansi_term::Colour::RGB;
use chrono::Local;
use fern::Dispatch;
use log::{LevelFilter, Log};
use regex::Regex;
use sentry::types::Dsn;
use sentry_log::{NoopLogger, SentryLogger};
use serde_json::json;

use crate::{config::Config, constants};

#[allow(dead_code)]
const ANSI_TERM_REGEX: &str = r#"\u001b\[.*?m"#;

pub fn setup_sentry(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(dsn) = &config.sentry_dsn {
        debug!("Sentry DSN was provided! Now enabling...");
        let _ = sentry::init(sentry::ClientOptions {
            dsn: Some(Dsn::from_str(dsn.as_str())?),
            ..Default::default()
        });
    }

    Ok(())
}

pub fn setup_logging(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let default_level = &"info".to_string();
    let sentry_enabled = config.sentry_dsn.is_some();

    let level = if let Some(log) = &config.logging {
        let l = log.level.as_ref();
        l.unwrap_or(default_level)
    } else {
        default_level
    };

    let is_json = match &config.logging {
        Some(cfg) => cfg.json.unwrap_or(false),
        None => false,
    };

    let log_level = match level.as_str() {
        "off" => log::LevelFilter::Off,
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    let dispatch = Dispatch::new()
        .format(move |out, message, record| {
            let thread = std::thread::current();
            let name = thread.name().unwrap_or("main");

            if var("TELEMETRY_DISABLE_COLOURS").is_ok() {
                if is_json {
                    let level_name = record.level().as_str();
                    let regex = Regex::new(ANSI_TERM_REGEX).unwrap();
                    let msg = regex.replace_all(format_args!("{}", message).to_string().as_str(), "").to_string();
                    let data = json!({
                        "@timestamp": Local::now().to_rfc3339(),
                        "@version": "1",
                        "message": msg,
                        "source": format!("Noelware/telemetry-server v{}", constants::VERSION),
                        "module": record.target(),
                        "thread": name,
                        "level": level_name,
                        "file": json!({
                            "path": record.file(),
                            "line": record.line()
                        })
                    });

                    out.finish(format_args!("{}", data));
                } else {
                    out.finish(format_args!(
                        "{} {:<5} [{} <{}>] :: {}",
                        Local::now().format("[%B %d, %G | %H:%M:%S %p]"),
                        record.level(),
                        record.target(),
                        name,
                        message
                    ));
                }
            } else if is_json {
                let level_name = record.level().as_str();
                let msg = format_args!("{}", message).to_string();

                let data = json!({
                    "@timestamp": Local::now().to_rfc3339(),
                    "@version": "1",
                    "message": msg.as_str(),
                    "source": format!("Noelware/telemetry-server v{}", constants::VERSION),
                    "module": record.target(),
                    "thread": name,
                    "level": level_name,
                    "file": json!({
                        "path": record.file(),
                        "line": record.line()
                    })
                });

                out.finish(format_args!("{}", data));
            } else {
                let color = match record.level() {
                    log::Level::Error => RGB(153, 75, 104),
                    log::Level::Debug => RGB(163, 182, 138),
                    log::Level::Info => RGB(178, 157, 243),
                    log::Level::Trace => RGB(163, 182, 138),
                    log::Level::Warn => RGB(243, 243, 134),
                };

                out.finish(format_args!(
                    "{} {} {}{}{} :: {}",
                    RGB(134, 134, 134).paint(format!(
                        "{}",
                        Local::now().format("[%B %d, %G | %H:%M:%S %p]")
                    )),
                    color.paint(format!("{:<5}", record.level())),
                    RGB(178, 157, 243).paint(format!("[{} ", record.target())),
                    RGB(255, 105, 189).paint(format!("<{}>", name)),
                    RGB(178, 157, 243).paint("]"),
                    message
                ));
            }
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(if sentry_enabled {
            Box::new(SentryLogger::default()) as Box<dyn Log>
        } else {
            Box::new(NoopLogger) as Box<dyn Log>
        })
        .chain(match &config.logging {
            Some(config) => match &config.logstash_uri {
                Some(endpoint) => {
                    let parsed_host = endpoint
                        .parse::<SocketAddr>()
                        .expect("Unable to parse Logstash endpoint to SocketAddr.");

                    let stream = TcpStream::connect(parsed_host).expect("Unable to connect to TCP stream. Did you configure the Logstash TCP input correctly?");
                    Dispatch::new()
                        .format(move |out, message, record| {
                            // If we're already in JSON logging, then we should just print it out
                            // since the parent dispatch does it already.
                            if is_json {
                                out.finish(format_args!("{}", message));
                                return;
                            }

                            let thread = std::thread::current();
                            let thread_name = thread.name().unwrap_or("main");
                            let level_name = record.level().as_str();
                            let regex = Regex::new(ANSI_TERM_REGEX).unwrap();
                            let msg = regex.replace_all(format_args!("{}", message).to_string().as_str(), "").to_string();

                            let inner_message_regex = Regex::new(r#"\[(\w.+)\] :: "#).unwrap();
                            let raw_message = inner_message_regex.replace_all(msg.as_str(), "").to_string();
                            let data = json!({
                                "@timestamp": Local::now().to_rfc3339(),
                                "@version": "1",
                                "message": raw_message,
                                "source": format!("Noelware/telemetry-server v{}", constants::VERSION),
                                "module": record.target(),
                                "thread": thread_name,
                                "level": level_name,
                                "file": json!({
                                    "path": record.file(),
                                    "line": record.line()
                                })
                            });

                            out.finish(format_args!("{}", data));
                        })
                        .chain(Box::new(stream) as Box<dyn Write + Send>)
                        .level(log_level)
                }
                None => Dispatch::new().level(LevelFilter::Off) // this will be dropped since it has no children and the log filter is Off.
            },
            None => Dispatch::new().level(LevelFilter::Off), // this will be dropped since it has no children and the log filter is Off.
        });

    dispatch.apply()?;
    Ok(())
}

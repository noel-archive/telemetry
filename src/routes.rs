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

use std::time::SystemTime;

use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use clickhouse_rs::Block;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    clickhouse::ClickHouse,
    responses::{self, respond, ApiResponse, Empty},
    telemetry::TelemetryServer,
};

#[derive(Serialize, Debug)]
struct MainResponse {
    message: String,
}

#[derive(Serialize, Debug)]
struct StatsResponse {
    db_calls: usize,
    events_emitted: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackBody {
    product: String,
    vendor: String,
    arch: String,
    os: String,
    version: String,
    distribution: String,
    data: Value,
}

pub async fn home() -> impl Responder {
    HttpResponse::Ok().json(respond(MainResponse {
        message: "hello, world.".into(),
    }))
}

pub async fn stats(data: web::Data<TelemetryServer>) -> impl Responder {
    let clickhouse = data.clickhouse.clone();
    let calls = ClickHouse::calls();

    let events_emitted = clickhouse
        .query("SELECT COUNT(*) FROM telemetry.events", |block| {
            block.get::<u64, _>(0, 0).unwrap_or(0)
        })
        .await;

    if events_emitted.is_err() {
        return HttpResponse::InternalServerError().json(responses::error(
            "UNKNOWN_ERROR",
            "Unknown exception had occurred while collecting statistics. :(",
        ));
    }

    HttpResponse::Ok().json(respond(StatsResponse {
        db_calls: calls,
        events_emitted: events_emitted.unwrap(),
    }))
}

// But, how can we not forge data? Well, I will tell you.

pub async fn send(
    mut data_payload: web::Payload,
    data: web::Data<TelemetryServer>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = data_payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > 262_144 {
            return Err(actix_web::error::ErrorPayloadTooLarge(format!(
                "{:#?}",
                serde_json::to_string::<ApiResponse<Empty>>(&ApiResponse::<Empty> {
                    success: false,
                    data: None,
                    errors: Some(vec![responses::Error::new("", "")]),
                }),
            )));
        }

        body.extend_from_slice(&chunk);
    }

    let payload = serde_json::from_slice::<TrackBody>(&body)?;
    let clickhouse = data.clickhouse.clone();
    let now = SystemTime::now();
    let now_in_utc: DateTime<Utc> = now.into();

    let payload_to_ch = json!({
        "distribution": payload.distribution,
        "version": payload.version,
        "arch": payload.arch,
        "os": payload.os,
        "data": payload.data,
        "fired_at": now_in_utc.to_rfc3339()
    });

    let mut snowflake = data.snowflake.clone();
    let id = snowflake.generate();
    let block = Block::new()
        .column("Data", vec![serde_json::to_string(&payload_to_ch).unwrap()])
        .column("ID", vec![id as u64])
        .column("Product", vec![payload.product])
        .column("Vendor", vec![payload.vendor]);

    clickhouse
        .insert("events", block.clone())
        .await
        .expect("Unable to insert into ClickHouse");

    Ok(HttpResponse::Created().json(ApiResponse::<Empty> {
        success: true,
        data: None,
        errors: None,
    }))
}

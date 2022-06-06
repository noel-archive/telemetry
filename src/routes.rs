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

use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::{
    responses::{self, respond},
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

pub async fn home() -> impl Responder {
    HttpResponse::Ok().json(respond(MainResponse {
        message: "hello, world.".into(),
    }))
}

pub async fn stats(data: web::Data<TelemetryServer>) -> impl Responder {
    let clickhouse = data.clickhouse.clone();
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
        db_calls: 0,
        events_emitted: events_emitted.unwrap(),
    }))
}

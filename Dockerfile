# 🐻‍❄️🌧️ Noelware Telemetry: Telemetry project for Noelware to capture anonymous data about our running products.
# Copyright 2022 Noelware <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM rustlang/rust:nightly-alpine3.15 AS builder

RUN apk update && apk add --no-cache git ca-certificates build-base openssl-dev
WORKDIR /build

COPY Cargo.toml .
RUN echo "fn main() {}" >> dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
ENV RUSTFLAGS=-Ctarget-feature=-crt-static \
  CARGO_INCREMENTAL=1

RUN cargo build --release
RUN rm dummy.rs && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .
RUN cargo build --release

FROM alpine:3.16

RUN apk update && apk add --no-cache openssl build-base bash tini
WORKDIR /app/noelware/telemetry
COPY --from=builder /build/target/release/telemetry-server .

USER 1001
ENTRYPOINT ["tini", "-s"]
CMD ["/app/noelware/telemetry/telemetry-server"]

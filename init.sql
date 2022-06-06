/*
 * 🐻‍❄️🌧️ Noelware Telemetry: Telemetry project for Noelware, to capture anonymous data about the running products.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*********************************************************************************************\
|============================================================================================|
|                       TELEMETRY SERVER ~ CLICKHOUSE INIT SQL FILE                          |
|                                                                                            |
| To use this, you must run:                                                                 |
|   $ clickhouse-client --multiquery --queries-file init.sql                                 |
|============================================================================================|
\**********************************************************************************************/

DROP TABLE IF EXISTS telemetry.events;

SET allow_experimental_object_type = 1;
CREATE TABLE IF NOT EXISTS telemetry."events"(
    -- At what time this telemetry event was fired at.
    FiredAt DateTime,

    -- The data object that is used.
    Data JSON,

    -- The ID of the telemetry event.
    ID UInt64,

    -- The product that was used for this telemetry event.
    Product String,

    -- The vendor, always "Noelware"
    Vendor String
) ENGINE=MergeTree() PARTITION BY toYYYYMM(FiredAt) ORDER BY (ID, Product, Vendor, FiredAt);

-- -- Use this line for replication.
-- CREATE TABLE IF NOT EXISTS telemetry."events"(
--     -- At what time this telemetry event was fired at.
--     FiredAt DateTime,

--     -- The data object that is used.
--     Data JSON,

--     -- The ID of the telemetry event.
--     ID UInt64,

--     -- The product that was used for this telemetry event.
--     Product String,

--     -- The vendor, always "Noelware"
--     Vendor String,

--     -- The version.
--     version String,
-- ) ENGINE=ReplicatedMergeTree('/clickhouse/tables/{layer}-{shard}/telemetry', '{replica}', version) PARTITION BY toYYYYMM(FiredAt) ORDER BY (ID, Product, Vendor, FiredAt);

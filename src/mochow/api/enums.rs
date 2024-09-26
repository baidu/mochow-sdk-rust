/*
 * Copyright 2024 Baidu, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 * except in compliance with the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
 * either express or implied. See the License for the specific language governing permissions
 * and limitations under the License.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutoBuildPolicyType {
    #[serde(rename = "TIMING", alias = "timing")]
    TIMING,
    #[serde(rename = "PERIODICAL", alias = "periodical")]
    PERIODICAL,
    #[serde(rename = "ROW_COUNT_INCREMENT", alias = "row_count_increment")]
    ROW_COUNT_INCREMENT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "BOOL")]
    BOOL,
    #[serde(rename = "INT8")]
    INT8,
    #[serde(rename = "UINT8")]
    UINT8,
    #[serde(rename = "INT16")]
    Int16,
    #[serde(rename = "UINT16")]
    Uint16,
    #[serde(rename = "INT32")]
    INT32,
    #[serde(rename = "UINT32")]
    UINT32,
    #[serde(rename = "INT64")]
    INT64,
    #[serde(rename = "UINT64")]
    UINT64,
    #[serde(rename = "FLOAT")]
    FLOAT,
    #[serde(rename = "DOUBLE")]
    DOUBLE,
    #[serde(rename = "DATE")]
    DATE,
    #[serde(rename = "DATETIME")]
    DATETIME,
    #[serde(rename = "TIMESTAMP")]
    TIMESTAMP,
    #[serde(rename = "STRING")]
    STRING,
    #[serde(rename = "BINARY")]
    BINARY,
    #[serde(rename = "UUID")]
    UUID,
    #[serde(rename = "TEXT")]
    TEXT,
    #[serde(rename = "TEXT_GBK")]
    TEXT_GBK,
    #[serde(rename = "TEXT_GB18030")]
    TEXT_GB18030,
    #[serde(rename = "FLOAT_VECTOR")]
    FLOAT_VECTOR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexState {
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "BUILDING")]
    BUILDING,
    #[serde(rename = "NORMAL")]
    NORMAL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableState {
    #[serde(rename = "INVALID")]
    INVALID,
    #[serde(rename = "CREATING")]
    CREATING,
    #[serde(rename = "NORMAL")]
    NORMAL,
    #[serde(rename = "DELETING")]
    DELETING,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexType {
    #[serde(rename = "FLAT")]
    FLAT,
    #[serde(rename = "HNSW")]
    HNSW,
    #[serde(rename = "HNSWPQ")]
    HNSWPQ,
    #[serde(rename = "PUCK")]
    PUCK,
    #[serde(rename = "SECONDARY")]
    SECONDARY_INDEX,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    #[serde(rename = "L2")]
    L2,
    #[serde(rename = "IP")]
    IP,
    #[serde(rename = "COSINE")]
    COSINE,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum PartitionType {
    #[default]
    #[serde(rename = "HASH")]
    HASH,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ReadConsistency {
    #[default]
    #[serde(rename = "EVENTUAL")]
    EVENTUAL,
    #[serde(rename = "STRONG")]
    STRONG,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerErrorCode {
    UNKNOWN,
    INTERNAL_ERROR = 1,
    INVALID_PARAMETER = 2,
    INVALID_HTTP_URL = 10,
    INVALID_HTTP_HEADER = 11,
    INVALID_HTTP_BODY = 12,
    MISS_SSL_CERTIFICATES = 13,

    USER_NOT_EXIST = 20,
    USER_ALREADY_EXIST = 21,
    ROLE_NOT_EXIST = 22,
    ROLE_ALREADY_EXIST = 23,
    AUTHENTICATION_FAILED = 24,
    PERMISSION_DENIED = 25,

    DB_NOT_EXIST = 50,
    DB_ALREADY_EXIST = 51,
    DB_TOO_MANY_TABLES = 52,
    DB_NOT_EMPTY = 53,

    INVALID_TABLE_SCHEMA = 60,
    INVALID_PARTITION_PARAMETERS = 61,
    TABLE_TOO_MANY_FIELDS = 62,
    TABLE_TOO_MANY_FAMILIES = 63,
    TABLE_TOO_MANY_PRIMARY_KEYS = 64,
    TABLE_TOO_MANY_PARTITION_KEYS = 65,
    TABLE_TOO_MANY_VECTOR_FIELDS = 66,
    TABLE_TOO_MANY_INDEXES = 67,
    DYNAMIC_SCHEMA_ERROR = 68,
    TABLE_NOT_EXIST = 69,
    TABLE_ALREADY_EXIST = 70,
    INVALID_TABLE_STATE = 71,
    TABLE_NOT_READY = 72,
    ALIAS_NOT_EXIST = 73,
    ALIAS_ALREADY_EXIST = 74,

    FIELD_NOT_EXIST = 80,
    FIELD_ALREADY_EXIST = 81,
    VECTOR_FIELD_NOT_EXIST = 82,

    INVALID_INDEX_SCHEMA = 90,
    INDEX_NOT_EXIST = 91,
    INDEX_ALREADY_EXIST = 92,
    INDEX_DUPLICATED = 93,
    INVALID_INDEX_STATE = 94,

    PRIMARY_KEY_DUPLICATED = 100,
    ROW_KEY_NOT_FOUND = 101,
}

impl From<i32> for ServerErrorCode {
    fn from(value: i32) -> Self {
        match value {
            1 => ServerErrorCode::INTERNAL_ERROR,
            2 => ServerErrorCode::INVALID_PARAMETER,
            10 => ServerErrorCode::INVALID_HTTP_URL,
            11 => ServerErrorCode::INVALID_HTTP_HEADER,
            12 => ServerErrorCode::INVALID_HTTP_BODY,
            13 => ServerErrorCode::MISS_SSL_CERTIFICATES,
            20 => ServerErrorCode::USER_NOT_EXIST,
            21 => ServerErrorCode::USER_ALREADY_EXIST,
            22 => ServerErrorCode::ROLE_NOT_EXIST,
            23 => ServerErrorCode::ROLE_ALREADY_EXIST,
            24 => ServerErrorCode::AUTHENTICATION_FAILED,
            25 => ServerErrorCode::PERMISSION_DENIED,
            50 => ServerErrorCode::DB_NOT_EXIST,
            51 => ServerErrorCode::DB_ALREADY_EXIST,
            52 => ServerErrorCode::DB_TOO_MANY_TABLES,
            53 => ServerErrorCode::DB_NOT_EMPTY,
            60 => ServerErrorCode::INVALID_TABLE_SCHEMA,
            61 => ServerErrorCode::INVALID_PARTITION_PARAMETERS,
            62 => ServerErrorCode::TABLE_TOO_MANY_FIELDS,
            63 => ServerErrorCode::TABLE_TOO_MANY_FAMILIES,
            64 => ServerErrorCode::TABLE_TOO_MANY_PRIMARY_KEYS,
            65 => ServerErrorCode::TABLE_TOO_MANY_PARTITION_KEYS,
            66 => ServerErrorCode::TABLE_TOO_MANY_VECTOR_FIELDS,
            67 => ServerErrorCode::TABLE_TOO_MANY_INDEXES,
            68 => ServerErrorCode::DYNAMIC_SCHEMA_ERROR,
            69 => ServerErrorCode::TABLE_NOT_EXIST,
            70 => ServerErrorCode::TABLE_ALREADY_EXIST,
            71 => ServerErrorCode::INVALID_TABLE_STATE,
            72 => ServerErrorCode::TABLE_NOT_READY,
            73 => ServerErrorCode::ALIAS_NOT_EXIST,
            74 => ServerErrorCode::ALIAS_ALREADY_EXIST,
            80 => ServerErrorCode::FIELD_NOT_EXIST,
            81 => ServerErrorCode::FIELD_ALREADY_EXIST,
            82 => ServerErrorCode::VECTOR_FIELD_NOT_EXIST,
            90 => ServerErrorCode::INVALID_INDEX_SCHEMA,
            91 => ServerErrorCode::INDEX_NOT_EXIST,
            92 => ServerErrorCode::INDEX_ALREADY_EXIST,
            93 => ServerErrorCode::INDEX_DUPLICATED,
            94 => ServerErrorCode::INVALID_INDEX_STATE,
            100 => ServerErrorCode::PRIMARY_KEY_DUPLICATED,
            101 => ServerErrorCode::ROW_KEY_NOT_FOUND,
            _ => ServerErrorCode::UNKNOWN,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serialize() {
        let p = super::AutoBuildPolicyType::TIMING;

        let s = serde_json::to_string(&p).unwrap();

        assert_eq!(s, "\"TIMING\"");
    }
}

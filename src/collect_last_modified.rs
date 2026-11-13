//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

use serde_json::Value;

/// Collect all `lastModified` keys from a `Value`.
pub fn collect_last_modified(
    val: &Value,
    out: &mut Vec<u64>,
) {
    match *val {
        Value::Object(ref map) => {
            for (key, value) in map {
                if key == "lastModified" {
                    if let Some(last_modified) = value.as_u64() {
                        out.push(last_modified);
                    }
                } else {
                    collect_last_modified(value, out);
                }
            }
        }
        Value::Array(ref items) => {
            for item in items {
                collect_last_modified(item, out);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

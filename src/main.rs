//! SPDX-License-Identifier: GPL-3.0-or-later
//! SPDX-FileCopyrightText: Stefan Rankovic <stefi.rankovic@proton.me>

mod collect_last_modified;
mod consts;

use collect_last_modified::collect_last_modified;
use consts::{colors, time};
use core::{error::Error, time::Duration};
use serde_json::Value;
use std::{
    env::{self, VarError},
    fs::{self, read_to_string},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::consts::symbols;

fn main() {
    let _: Result<(), Box<dyn Error>> = run();
}

/// Internal wrapper over `main()` that returns a `Result`.
/// Exists because `main()` shouldn't return `Result`.
fn run() -> Result<(), Box<dyn Error>> {
    let pwd: PathBuf = env::current_dir()?;
    let flake_lock_path: PathBuf = pwd.join("flake.lock");
    let current_time: SystemTime = SystemTime::now();

    // First, check the modified time. If it is unacceptable, print a warning and return.
    // But the modification time doesn't tell the whole story.
    // If there was no warning, then proceed to the next part.
    // The next part checks the `lastModified` keys inside the `flake.lock`.
    if let Ok(mtime) = fs::metadata(&flake_lock_path).and_then(|md| md.modified()) {
        let age: u64 = current_time.duration_since(mtime)?.as_secs();
        #[expect(clippy::print_stderr, reason = "Intentional.")]
        #[expect(
            clippy::integer_division,
            reason = "The remainder is intentionally ignored. Nobody cares whether it was 35.2 or 35.7 days ago."
        )]
        if age >= time::MIN_UNACCEPTED_AGE {
            eprintln!(
                "{}{} Warning: `flake.lock` in current directory not updated for {} days. It is probably outdated.{}",
                colors::YELLOW,
                symbols::WARNING,
                age / time::ONE_DAY,
                colors::RESET
            );
            return Ok(());
        };
    }

    let string: String = read_to_string(pwd.join("flake.lock"))?;
    let json: Value = serde_json::from_str(&string)?;
    let instances: Vec<u64> = {
        let mut inner: Vec<u64> = Vec::new();
        collect_last_modified(&json, &mut inner);
        inner.sort_unstable();
        inner.reverse();
        inner
    };
    // A `VarError` is obviously nonsense in this context, but it doesn't matter.
    // The error is going to be ignored anyway, so `VarError` was the simplest to construct.
    let newest_instance: u64 = *instances.first().ok_or(VarError::NotPresent)?;
    let newest_instance_age: u64 = current_time
        .duration_since(UNIX_EPOCH + Duration::from_secs(newest_instance))?
        .as_secs();
    #[expect(clippy::print_stderr, reason = "Intentional.")]
    #[expect(
        clippy::integer_division,
        reason = "The remainder is intentionally ignored. Nobody cares whether it was 35.2 or 35.7 days ago."
    )]
    if newest_instance_age >= time::MIN_UNACCEPTED_AGE {
        eprintln!(
            "{}{} Warning: The newest input inside the file `flake.lock` is {} days old. The file is probably outdated.{}",
            colors::YELLOW,
            symbols::WARNING,
            newest_instance_age / time::ONE_DAY,
            colors::RESET,
        );
    }

    Ok(())
}

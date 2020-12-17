use byte_unit::Byte;
use chrono::{DateTime, SecondsFormat, Utc};
use glob::glob;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use tabular::{Row, Table};

fn ls(file_pattern: &str) -> IOResult<()> {
    let mut table = Table::new("{:<} | {:<} | {:<}");
    let all_results = glob(file_pattern);

    if all_results.is_err() {
        return Err(IOError::new(IOErrorKind::PermissionDenied, "Bad path!"));
    }

    let all_results = all_results.unwrap();

    for entry_result in all_results {
        if entry_result.is_err() {
            continue;
        }

        let entry = entry_result.unwrap();
        let metadata = entry.metadata()?;
        let modified_time: DateTime<Utc> = metadata.modified().unwrap().into();
        table.add_row(
            Row::new()
                .with_cell(modified_time.to_rfc3339_opts(SecondsFormat::Millis, true))
                .with_cell(format!(
                    "{:>6}",
                    Byte::from_bytes(metadata.len() as u128)
                        .get_appropriate_unit(true)
                        .to_string()
                ))
                .with_cell(entry.canonicalize().unwrap().display()),
        );
    }

    print!("{}", table);

    Ok(())
}

fn main() {
    ls(".bash_history").unwrap();
}

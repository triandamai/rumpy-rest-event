use super::multipart_file::{MultiFileExtractor, SingleFileExtractor};
use crate::common::api_response::ApiResponse;
use bson::oid::ObjectId;
use chrono::{Local, NaiveDate, TimeZone, Utc};
use log::info;
use mime::Mime;
use rand::Rng;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use validator::ValidationError;

pub const QUERY_LOWEST: &str = "lowest";
pub const QUERY_HIGHEST: &str = "highest";
pub const QUERY_ASC: &str = "asc";
pub const QUERY_DESC: &str = "desc";
pub const QUERY_LATEST: &str = "latest";
pub const QUERY_OLDEST: &str = "oldest";

// Function to map MIME types to file extensions
pub fn get_extension_from_mime(mime: &Mime) -> Option<&'static str> {
    match mime.as_ref() {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/gif" => Some("gif"),
        "application/pdf" => Some("pdf"),
        "text/plain" => Some("txt"),
        "application/zip" => Some("zip"),
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => Some("docx"),
        "application/msword" => Some("doc"),
        "audio/mpeg" => Some("mp3"),
        "video/mp4" => Some("mp4"),
        // Add more MIME types here as needed
        _ => None, // Return None if no matching extension is found
    }
}

pub fn get_mime_type_from_filename(filename: &str) -> &'static str {
    if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
        "image/jpeg"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".txt") {
        "text/plain"
    } else if filename.ends_with(".pdf") {
        "application/pdf"
    } else {
        "application/octet-stream"
    }
}

pub fn vec_to_array<const N: usize, K: Debug>(vec: Vec<K>) -> Option<[K; N]> {
    if vec.len() == N {
        Some(vec.try_into().unwrap())
    } else {
        None
    }
}

pub fn create_or_new_object_id(id: &str) -> Option<ObjectId> {
    let id = ObjectId::from_str(id);
    if let Ok(value) = id {
        Some(value)
    } else {
        Some(ObjectId::new())
    }
}
pub fn create_object_id_option(id: &str) -> Option<ObjectId> {
    let id = ObjectId::from_str(id);
    if let Ok(value) = id {
        Some(value)
    } else {
        None
    }
}

pub fn validate_date_of_birth_option(date: &&String) -> Result<(), ValidationError> {
    let text = date;
    let parse = NaiveDate::parse_from_str(text, "%Y-%m-%d");
    match parse {
        Ok(_) => Ok(()),
        Err(e) => {
            info!(target: "validate_dob::","{:?}",e);
            Err(
                ValidationError::new("Invalid date of birth format: YYYY-MM-DD")
                    .with_message(Cow::from("Invalid date of birth format: YYYY-MM-DD")),
            )
        }
    }
}

pub fn validate_single_extractor(file: &SingleFileExtractor) -> Result<(), ValidationError> {
    if file.is_error {
        return Err(ValidationError::new("file").with_message(Cow::from("Limit file exceeded")));
    }
    Ok(())
}
pub fn validate_multi_extractor(file: &MultiFileExtractor) -> Result<(), ValidationError> {
    if file.is_error {
        return Err(ValidationError::new("file").with_message(Cow::from("Limit file exceeded")));
    }
    Ok(())
}

pub fn validate_date_of_birth(date: &String) -> Result<(), ValidationError> {
    let parse = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    match parse {
        Ok(_) => Ok(()),
        Err(e) => {
            info!(target: "validate_dob::","{:?}",e);
            Err(
                ValidationError::new("Invalid date of birth format: YYYY-MM-DD")
                    .with_message(Cow::from("Invalid date of birth format: YYYY-MM-DD")),
            )
        }
    }
}

pub fn validate_gender(gender: &String) -> Result<(), ValidationError> {
    if gender == "M" {
        return Ok(());
    }

    if gender == "F" {
        return Ok(());
    }

    Err(
        ValidationError::new("Invalid Gender, valid value M(Male) or F(Female)").with_message(
            Cow::from("Invalid Gender, valid value M(Male) or F(Female)"),
        ),
    )
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn generate_member_code(prefix: &str) -> String {
    // Get the current timestamp in seconds since UNIX epoch
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Increment a small counter for extra uniqueness
    let counter = COUNTER.fetch_add(1, Ordering::SeqCst) % 1000; // Keep counter to 3 digits

    // Format as: PREFIX + short timestamp + counter
    format!(
        "{}{:X}{:03}",
        get_first_name(prefix).unwrap_or(""),
        timestamp % 0xFFFFFF,
        counter
    )
}

pub fn get_first_name(full_name: &str) -> Option<&str> {
    full_name.split_whitespace().next()
}

pub fn to_hashmap<Id: Eq + Hash, T: Copy>(data: Vec<T>, id: fn(T) -> Id) -> HashMap<Id, T> {
    let mut hash = HashMap::new();
    for thread in data {
        let id = id(thread);
        if !hash.contains_key(&id) {
            hash.insert(id, thread);
        }
    }
    hash
}

pub fn generate_otp() -> String {
    let mut rng = rand::rng();
    let otp: u32 = rng.random_range(100_000..1_000_000); // ensures a 6-digit number
    otp.to_string()
}

//hashmap
pub fn get_naive_date_time(session: Option<&String>) -> chrono::DateTime<chrono::Utc> {
    session.map_or_else(
        || Local::now().naive_local().and_utc(),
        |value| match value.parse::<i64>() {
            Ok(timestamp) => chrono::DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_else(|| Local::now().naive_local().and_utc()),
            Err(_) => Local::now().naive_local().and_utc(),
        },
    )
}

pub fn get_string_with_default(session: Option<&String>) -> String {
    let default_string = String::new();
    session.map_or_else(|| default_string, |value| value.clone())
}

pub fn get_i64_with_default(session: Option<&String>) -> i64 {
    let default_string = String::new();
    session
        .unwrap_or(&default_string)
        .parse::<i64>()
        .unwrap_or_else(|_| 0)
}

pub fn string_to_bson_datetime(date: String) -> Result<bson::DateTime, String> {
    let chrono_date = string_to_chrono_datetime(&date);
    if let Err(why) = chrono_date {
        return Err(why);
    }
    let chrono_date = chrono_date.unwrap();
    let date = bson::datetime::DateTime::from_chrono(chrono_date);
    Ok(date)
}

pub fn string_to_chrono_datetime(date: &str) -> Result<chrono::DateTime<Utc>, String> {
    // Step 1: Parse as NaiveDate (date without time)
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    if naive_date.is_err() {
        info!(target:"daily::report::err","{:?}",naive_date.unwrap_err());
        return Err(format!("{:?}", naive_date.unwrap_err()));
    }
    let naive_dt = naive_date.unwrap();
    // Step 2: Convert to NaiveDateTime by adding time 00:00:00
    let naive_dt = naive_dt.and_hms_opt(0, 0, 0).unwrap();

    let datetime = chrono::Utc.from_local_datetime(&naive_dt).single();
    // Step 3: Convert to DateTime<Utc>
    if datetime.is_none() {
        info!(target:"daily::report::err","Can't get datetime");
        return Err(format!("{:?}", naive_date.unwrap_err()));
    }
    Ok(datetime.unwrap())
}

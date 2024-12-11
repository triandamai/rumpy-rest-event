use bson::oid::ObjectId;
use chrono::NaiveDate;
use log::info;
use mime::Mime;
use std::borrow::Cow;
use std::fmt::Debug;
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
    format!("{}{:X}{:03}", get_first_name(prefix).unwrap_or(""), timestamp % 0xFFFFFF, counter)
}

pub fn get_first_name(full_name: &str) -> Option<&str> {
    full_name.split_whitespace().next()
}
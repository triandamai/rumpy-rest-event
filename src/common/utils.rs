use std::fmt::Debug;
use std::str::FromStr;
use bson::oid::ObjectId;
use mime::Mime;


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

pub fn vec_to_array<const N: usize,K:Debug>(vec: Vec<K>) -> Option<[K; N]> {
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

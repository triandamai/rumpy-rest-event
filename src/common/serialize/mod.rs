use serde::Serializer;

pub fn serialize_to_redact_password<S>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_some() {
        serializer.serialize_str("*****")
    // Convert ObjectId to a hex string
    } else {
        serializer.serialize_none()
    }
}

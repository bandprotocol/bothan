pub(crate) fn into_key(source_id: &str, id: &str) -> String {
    format!("{}{}", source_id, id)
}

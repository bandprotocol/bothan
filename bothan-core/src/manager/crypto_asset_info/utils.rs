use std::fmt::Display;

pub fn into_key<T: Display, U: Display>(source_id: &T, id: &U) -> String {
    format!("{}-{}", source_id, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_key() {
        let source_id = "source_id";
        let id = "id";
        let key = into_key(&source_id, &id);
        assert_eq!(key, "source_id-id");
    }
}

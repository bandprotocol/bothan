use crate::VERSION;
use semver::{Version, VersionReq};
use std::fmt::Display;

#[macro_export]
macro_rules! price {
    ($signal_id:expr, $status:expr, $price:expr) => {
        Price {
            signal_id: $signal_id.clone(),
            status: $status.into(),
            price: $price,
        }
    };
}

pub fn into_key<T: Display, U: Display>(source_id: &T, id: &U) -> String {
    format!("{}-{}", source_id, id)
}

pub fn valid_version(version: Version) -> bool {
    // This should never fail, hence unwrap here
    VersionReq::parse(&format!("<={}", VERSION))
        .unwrap()
        .matches(&version)
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

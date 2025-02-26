use rand::RngCore;
use rand::rngs::OsRng;

pub fn create_uuid() -> String {
    let mut uuid_bytes = [0u8; 16];
    OsRng.fill_bytes(&mut uuid_bytes);
    hex::encode(uuid_bytes)
}

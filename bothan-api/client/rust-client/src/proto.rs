pub mod bothan {
    pub mod v1 {
        pub use bothan_service_client::BothanServiceClient;
        include!("proto/bothan.v1.rs");
    }
}

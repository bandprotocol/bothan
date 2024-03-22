macro_rules! arc_mutex {
    ($t:expr) => {
        std::sync::Arc::new(tokio::sync::Mutex::new($t))
    };
}

pub(crate) use arc_mutex;

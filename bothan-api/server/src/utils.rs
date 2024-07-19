macro_rules! arc_mutex {
    ($t:expr) => {
        std::sync::Arc::new(tokio::sync::Mutex::new($t))
    };
}

#[macro_export]
macro_rules! add_worker {
    ($manager:expr, $builder:ty, $config:expr) => {
        let worker_name = {
            let full_path = stringify!($config);
            let parts: Vec<&str> = full_path.split('.').collect();
            *parts.last().unwrap_or(&"")
        };
        let worker = <$builder>::new(Arc::new(bothan_core::store::Store::new()), $config.clone())
            .build()
            .await
            .expect(&format!("cannot build {} worker", worker_name));
        $manager.add_worker(worker_name.to_string(), worker).await;
    };
}

pub use add_worker;
pub(crate) use arc_mutex;

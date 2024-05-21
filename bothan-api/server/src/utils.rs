macro_rules! arc_mutex {
    ($t:expr) => {
        std::sync::Arc::new(tokio::sync::Mutex::new($t))
    };
}

#[macro_export]
macro_rules! add_service {
    ($manager:expr, $builder:ty, $config:expr) => {
        let service_name = {
            let full_path = stringify!($config);
            let parts: Vec<&str> = full_path.split('.').collect();
            *parts.last().unwrap_or(&"")
        };
        let service = <$builder>::new($config.clone())
            .build()
            .await
            .expect(&format!("cannot build {} service", service_name));
        $manager
            .add_service(service_name.to_string(), Box::new(service))
            .await;
    };
}

pub use add_service;
pub(crate) use arc_mutex;

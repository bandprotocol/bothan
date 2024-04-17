macro_rules! arc_mutex {
    ($t:expr) => {
        std::sync::Arc::new(tokio::sync::Mutex::new($t))
    };
}

macro_rules! add_service {
    ($manager:expr, $builder:ty, $config:expr) => {
        let service_name = {
            let full_path = stringify!($config);
            let parts: Vec<&str> = full_path.split('.').collect();
            *parts.last().unwrap_or(&"")
        };
        let service = <$builder>::new($config)
            .build()
            .await
            .expect(&format!("cannot build {} service", service_name));
        $manager
            .add_service(service_name.to_string(), Box::new(service))
            .await;
    };
}

pub(crate) use add_service;
pub(crate) use arc_mutex;

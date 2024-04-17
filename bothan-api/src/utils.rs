macro_rules! arc_mutex {
    ($t:expr) => {
        std::sync::Arc::new(tokio::sync::Mutex::new($t))
    };
}

macro_rules! add_service {
    ($manager:expr, $builder:ty, $service_name:expr, $config:expr) => {
        let service = <$builder>::new($config)
            .build()
            .await
            .expect(&format!("cannot build {} service", $service_name));
        $manager
            .add_service($service_name.to_string(), Box::new(service))
            .await;
    };
}

pub(crate) use add_service;
pub(crate) use arc_mutex;

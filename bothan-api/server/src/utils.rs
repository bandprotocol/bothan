#[macro_export]
macro_rules! add_worker {
    ($manager:expr, $store:expr, $builder:ty, $config:expr) => {
        let worker_name = {
            let full_path = stringify!($config);
            let parts: Vec<&str> = full_path.split('.').collect();
            parts.last().unwrap().to_string()
        };

        let worker_store = Arc::new(bothan_core::store::Store::create_worker_store(
            $store,
            &worker_name,
        ));

        let worker = <$builder>::new(worker_store, $config.clone())
            .build()
            .await
            .expect(&format!("cannot build {} worker", worker_name));

        $manager.add_worker(worker_name, worker).await;
    };
}

pub use add_worker;

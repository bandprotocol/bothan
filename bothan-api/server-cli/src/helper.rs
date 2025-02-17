pub trait Exitable<T> {
    fn exit_on_err(self, code: i32) -> T;
}

impl<T> Exitable<T> for anyhow::Result<T> {
    fn exit_on_err(self, code: i32) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(code);
            }
        }
    }
}

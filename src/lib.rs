pub mod bst;
use env_logger;
#[allow(unused_must_use)]
pub fn init_logger() {
    env_logger::builder().parse_default_env()
        .try_init();
}

pub mod bst;
#[allow(unused_must_use)]
pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .try_init();
}

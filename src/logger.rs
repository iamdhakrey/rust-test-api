pub fn logs() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}

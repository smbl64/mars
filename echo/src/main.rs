fn main() {
    // By default env_logger logs to stderr, which is what we want
    std::env::set_var(env_logger::DEFAULT_FILTER_ENV, "debug");
    env_logger::init();

    maelstrom_core::run();
}

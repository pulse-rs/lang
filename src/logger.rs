use vit_logger::{Config, VitLogger};

pub fn setup_logger(verbose: bool) {
    // Setup log's global logger
    std::env::set_var("RUST_LOG", if verbose { "trace" } else { "info" });
    VitLogger::new().init(
        Config::builder()
            .text(true)
            .target(verbose)
            .file(verbose)
            .line(verbose)
            .time(!verbose)
            .finish()
            .expect("Error building config"),
    );
}
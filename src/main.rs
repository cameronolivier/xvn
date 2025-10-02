use anyhow::Result;

fn main() -> Result<()> {
    // Only enable logging if RUST_LOG is explicitly set
    // In normal operation (release builds), users shouldn't see logs
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    xvn::cli::run()
}

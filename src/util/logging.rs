use std::env;
use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use env_logger::fmt::Color;
use log::Level;
use log::LevelFilter;

/// Initialises an env_logger with the specified [LevelFilter].
///
/// ```
/// use log::LevelFilter;
/// use prole::util::logging::init_logger;
///
/// init_logger(LevelFilter::Info);
/// ```
pub fn init_logger(level: LevelFilter) {
    // Initialise the logger at the default level of info
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", level.as_str())
    }
    // env_logger::init();
    Builder::from_default_env()
        .format(|buf, record| {
            let color = match record.level() {
                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Info => Color::Green,
                Level::Debug => Color::Magenta,
                Level::Trace => Color::Cyan,
            };
            let mut style = buf.style();
            style.set_color(color).set_bold(true);

            writeln!(buf,
                     "[{}] {} - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     style.value(record.level()),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

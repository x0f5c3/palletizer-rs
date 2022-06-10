use pretty_env_logger::env_logger::fmt::Color;
use std::io::Write;

/// Initialize the logging system with a pretty format.
///
/// Logging for the specified root module will be set to Error, Warn, Info, Debug or Trace, depending on the verbosity parameter.
/// Logging for all other modules is set to one level less verbose.
pub(crate) fn init(root_module: &str, extra_modules: &[&str], verbosity: i8) {
    let log_level = match verbosity {
        i8::MIN..=-2 => log::LevelFilter::Error,
        -1 => log::LevelFilter::Warn,
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        2..=i8::MAX => log::LevelFilter::Trace,
    };

    let extra_level = match verbosity {
        i8::MIN..=-1 => log::LevelFilter::Error,
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        3..=i8::MAX => log::LevelFilter::Trace,
    };

    let mut logger = pretty_env_logger::formatted_builder();

    logger.format(move |buffer, record: &log::Record| {
        let now = time::OffsetDateTime::now_utc();
        let mut date_style = buffer.style();
        let mut time_style = buffer.style();
        let mut target_style = buffer.style();
        let mut level_style = buffer.style();

        date_style.set_color(Color::Cyan);
        time_style.set_color(Color::Cyan);
        target_style.set_color(Color::Magenta);

        let level;
        match record.level() {
            log::Level::Trace => {
                level = "Trace";
                level_style.set_bold(true);
            }
            log::Level::Debug => {
                level = "Debug";
                level_style.set_bold(true);
            }
            log::Level::Info => {
                level = "Info";
                level_style.set_bold(true);
            }
            log::Level::Warn => {
                level = "Warn";
                level_style.set_color(Color::Yellow).set_bold(true);
            }
            log::Level::Error => {
                level = "Error";
                level_style.set_color(Color::Red).set_bold(true);
            }
        };
        let fmt1 = time::format_description::parse("%Y-%m-%d").unwrap();
        let fmt2 = time::format_description::parse("%H:%M:%S%.3f").unwrap();
        writeln!(
            buffer,
            "[{date} {time}] {level:<7} {message}",
            date = date_style.value(now.format(&fmt1).unwrap()),
            time = time_style.value(now.format(&fmt2).unwrap()),
            level = format_args!("[{}]", level_style.value(level)),
            message = record.args(),
        )
    });

    logger.filter_module(root_module, log_level);
    for module in extra_modules {
        logger.filter_module(module, extra_level);
    }

    logger.init();
}

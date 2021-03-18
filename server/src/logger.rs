#![allow(unused_must_use, dead_code, unused_variables, unused_imports)]
#![feature(ip)]

use crate::config;
use log::{
    LevelFilter,
    LevelFilter::{Debug, Info},
};
use std::str::FromStr;

pub fn init_logger(log_cfg: &config::log::Log) {
    use log4rs::{
        append::{
            console::ConsoleAppender,
            rolling_file::{
                policy::compound::{
                    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
                    CompoundPolicy,
                },
                RollingFileAppender,
            },
        },
        config::{Appender, Config, Root},
        encode::pattern::PatternEncoder,
    };
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h([{d(%Y-%m-%d %H:%M:%S)}][{l}][{T}] {m}{n})}",
        )))
        .build();
    let logfile = if cfg.enable_file {
        let directory = &cfg.file_directory;
        Some(
            RollingFileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(
                    "[{d(%Y-%m-%d %H:%M:%S)}][{l}][{T}][{M}:{L}] {m}{n}",
                )))
                .build(
                    directory.clone() + "/latest.log",
                    Box::new(CompoundPolicy::new(
                        Box::new(SizeTrigger::new(8 << 20)),
                        Box::new(
                            FixedWindowRoller::builder()
                                .base(1)
                                .build(&(directory.clone() + "/log-{}.gz"), 10)
                                .expect("Can't build FixedWindowRoller!"),
                        ),
                    )),
                )
                .expect("Can't build RollingFileAppender!"),
        )
    } else {
        None
    };

    let level = LevelFilter::from_str(&*cfg.level).unwrap_or(Info);

    let mut config = Config::builder();
    let mut root = Root::builder();
    config = config.appender(Appender::builder().build("stdout", Box::new(stdout)));
    root = root.appender("stdout");
    if let Some(logfile) = logfile {
        config = config.appender(Appender::builder().build("logfile", Box::new(logfile)));
        root = root.appender("logfile");
    }

    let config = config
        .build(root.build(level))
        .expect("Can't build log config!");

    log4rs::init_config(config).expect("Can't init log config!");
}

#[test]
fn test() {
    use log::{debug, error, info, trace, warn};
    init_logger(&config::log::Log::default());
    trace!("trace test");
    debug!("debug test");
    info!("info test");
    warn!("warn test");
    error!("error test");
}

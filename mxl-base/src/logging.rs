use crate::localization::helper::fl;
use anyhow::Result;
use log::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{LazyLock, OnceLock},
};

const DEFAULT_LEVEL: log::LevelFilter = log::LevelFilter::Trace;
const LOG_FILE_EXTENSION: &str = "log";

#[cfg(debug_assertions)] // Set debug level for console in debug builds
const CONSOLE_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

#[cfg(not(debug_assertions))] // Set debug level for console in release builds
const CONSOLE_LEVEL: log::LevelFilter = log::LevelFilter::Warn;

static LOG_RECEIVER_LOG_LEVEL: LazyLock<std::sync::RwLock<log::LevelFilter>> =
    LazyLock::new(|| std::sync::RwLock::new(DEFAULT_LEVEL));

pub fn set_log_level(level: log::LevelFilter) {
    *LOG_RECEIVER_LOG_LEVEL.write().unwrap() = level;
}

pub fn get_log_level() -> log::LevelFilter {
    *LOG_RECEIVER_LOG_LEVEL.read().unwrap()
}

static CURRENT_LOG_FILE_HOLDER: OnceLock<PathBuf> = OnceLock::new();
pub fn current_log_file() -> &'static PathBuf {
    CURRENT_LOG_FILE_HOLDER.get().expect("init() must be called first")
}

#[derive(Default)]
pub struct Builder {
    level_for: HashMap<&'static str, log::LevelFilter>,
    console_level_for: HashMap<&'static str, log::LevelFilter>,
    without_stderr: bool,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level_for(mut self, module: &'static str, level: log::LevelFilter) -> Self {
        self.level_for.insert(module, level);
        self
    }

    pub fn console_level_for(mut self, module: &'static str, level: log::LevelFilter) -> Self {
        self.console_level_for.insert(module, level);
        self
    }

    pub fn without_stderr(mut self) -> Self {
        self.without_stderr = true;
        self
    }

    fn build_with_panic_on_failure(&self, log_dir: &Path) {
        // NOTE!!!
        // Every error MUST be a panic here else the user will not be able to see the error!
        let mut logger = fern::Dispatch::new().level(DEFAULT_LEVEL);
        for (module, level) in &self.level_for {
            logger = logger.level_for(*module, *level);
        }
        logger = logger
            .filter(|metadata| metadata.level() <= *LOG_RECEIVER_LOG_LEVEL.read().unwrap())
            .format(|out, message, record| {
                out.finish(format_args!("{} [{}] {}", record.level(), record.target(), message))
            });
        let log_file = CURRENT_LOG_FILE_HOLDER
            .get_or_init(|| log_dir.join(format!("{}.{}", super::about::about().binary_name, LOG_FILE_EXTENSION)));

        std::fs::create_dir_all(log_dir).unwrap_or_else(|error| {
            panic!(
                "Cannot create logging directory '{}': {:?}",
                log_dir.to_string_lossy(),
                error
            )
        });
        logger = logger.chain(
            fern::log_file(log_file)
                .unwrap_or_else(|error| panic!("Cannot open log file '{}': {:?}", log_file.to_string_lossy(), error)),
        );
        if !self.without_stderr {
            let mut console_logger = fern::Dispatch::new().level(CONSOLE_LEVEL).chain(std::io::stderr());
            for (module, level) in &self.level_for {
                console_logger = console_logger.level_for(*module, *level);
            }
            for (module, level) in &self.console_level_for {
                console_logger = console_logger.level_for(*module, *level);
            }
            logger = logger.chain(console_logger);
        }
        logger.apply().expect("Cannot start logging");
    }

    pub fn build(self, log_dir: &Path) -> Result<()> {
        self.build_with_panic_on_failure(log_dir);
        let about = super::about::about();

        let log_file = current_log_file();
        if !self.without_stderr {
            // Currently not use translation. The log file name is wrapped to: <2068>log-file-name<2069>
            // Some terminals copy these possibly invisible special characters when selecting and copying,
            // so that the log file cannot be opened.
            if false {
                println!("{}", fl!("log-written-to", file_name = log_file.to_string_lossy()));
            } else {
                println!("Log is written to '{}'", log_file.to_string_lossy());
            }
        }

        info!("{} {}", about.app_name, about.version);
        info!("Log is written to '{}'", log_file.to_string_lossy());

        Ok(())
    }
}

use crate::localization::helper::fl;
use anyhow::Result;
use log::*;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{LazyLock, OnceLock},
};

const DEFAULT_LOG_FILE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;
const LOG_FILE_EXTENSION: &str = "log";

#[cfg(debug_assertions)] // Set debug level for console in debug builds
const DEFAULT_CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

#[cfg(not(debug_assertions))] // Set debug level for console in release builds
const DEFAULT_CONSOLE_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Warn;

static LOG_FILE_LOG_LEVEL: LazyLock<std::sync::RwLock<log::LevelFilter>> =
    LazyLock::new(|| std::sync::RwLock::new(DEFAULT_LOG_FILE_LOG_LEVEL));
pub fn set_log_file_log_level(level: log::LevelFilter) {
    *LOG_FILE_LOG_LEVEL.write().unwrap() = level;
}
pub fn get_log_file_log_level() -> log::LevelFilter {
    *LOG_FILE_LOG_LEVEL.read().unwrap()
}

static CONSOLE_LOG_LEVEL: LazyLock<std::sync::RwLock<log::LevelFilter>> =
    LazyLock::new(|| std::sync::RwLock::new(DEFAULT_CONSOLE_LOG_LEVEL));
pub fn set_console_log_level(level: log::LevelFilter) {
    *CONSOLE_LOG_LEVEL.write().unwrap() = level;
}
pub fn get_console_log_level() -> log::LevelFilter {
    *CONSOLE_LOG_LEVEL.read().unwrap()
}

static CURRENT_LOG_FILE_HOLDER: OnceLock<PathBuf> = OnceLock::new();
pub fn current_log_file() -> &'static PathBuf {
    CURRENT_LOG_FILE_HOLDER.get().expect("init() must be called first")
}

#[derive(Default)]
pub struct Builder {
    level_for: HashMap<&'static str, log::LevelFilter>,
    console_level_for: HashMap<&'static str, log::LevelFilter>,
    without_console: bool,
    dispatches: Vec<fern::Dispatch>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dispatch(mut self, dispatch: fern::Dispatch) -> Self {
        self.dispatches.push(dispatch);
        self
    }

    pub fn level_for(mut self, module: &'static str, level: log::LevelFilter) -> Self {
        self.level_for.insert(module, level);
        self
    }

    pub fn console_level_for(mut self, module: &'static str, level: log::LevelFilter) -> Self {
        self.console_level_for.insert(module, level);
        self
    }

    pub fn without_console(mut self) -> Self {
        self.without_console = true;
        self
    }

    fn add_log_level_for(
        mut logger: fern::Dispatch,
        levels_for: &HashMap<&'static str, log::LevelFilter>,
    ) -> fern::Dispatch {
        for (module, level) in levels_for {
            logger = logger.level_for(*module, *level);
        }
        logger
    }

    fn build_with_panic_on_failure(&mut self, log_dir: &Path) {
        // NOTE!!!
        // Every error MUST be a panic here else the user will not be able to see the error!

        let mut basic_logger = fern::Dispatch::new();
        basic_logger = Self::add_log_level_for(basic_logger, &self.level_for);

        {
            // log file logger
            let log_file = CURRENT_LOG_FILE_HOLDER
                .get_or_init(|| log_dir.join(format!("{}.{}", super::about::about().binary_name, LOG_FILE_EXTENSION)));
            std::fs::create_dir_all(log_dir).unwrap_or_else(|error| {
                panic!(
                    "Cannot create logging directory '{}': {:?}",
                    log_dir.to_string_lossy(),
                    error
                )
            });
            let mut file_logger = fern::Dispatch::new()
                .filter(|metadata| metadata.level() <= get_log_file_log_level())
                .format(|out, message, record| {
                    out.finish(format_args!("{} [{}] {}", record.level(), record.target(), message))
                })
                .chain(fern::log_file(log_file).unwrap_or_else(|error| {
                    panic!("Cannot open log file '{}': {:?}", log_file.to_string_lossy(), error)
                }));
            file_logger = Self::add_log_level_for(file_logger, &self.level_for);

            basic_logger = basic_logger.chain(file_logger)
        }

        if !self.without_console {
            // console logger
            let mut console_logger = fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!("{} [{}] {}", record.level(), record.target(), message))
                })
                .filter(|metadata| metadata.level() <= get_console_log_level())
                .chain(std::io::stderr());
            console_logger = Self::add_log_level_for(console_logger, &self.level_for);
            console_logger = Self::add_log_level_for(console_logger, &self.console_level_for);

            basic_logger = basic_logger.chain(console_logger);
        }
        for dispatch in std::mem::take(&mut self.dispatches) {
            basic_logger = basic_logger.chain(dispatch);
        }
        basic_logger.apply().expect("Cannot start logging");
    }

    pub fn build(mut self, log_dir: &Path) -> Result<()> {
        self.build_with_panic_on_failure(log_dir);
        let about = super::about::about();

        let log_file = current_log_file();
        if !self.without_console {
            // Currently not use translation. The log file name is wrapped to: <2068>log-file-name<2069>
            // Some terminals copy these possibly invisible special characters when selecting and copying,
            // so that the log file cannot be opened.
            if false {
                println!("{}", fl!("log-written-to", file_name = log_file.to_string_lossy()));
            } else {
                println!("Log is written to '{}'", log_file.to_string_lossy());
            }
        }

        info!("Application: {} Version: {}", about.app_name, about.version);

        Ok(())
    }
}

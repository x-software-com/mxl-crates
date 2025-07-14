use anyhow::{Context, Result};
use fs4::fs_std::FileExt;
use std::{
    fs::File,
    io::{Read, Write},
    panic,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};
use walkdir::WalkDir;
use zip::{ZipWriter, write::SimpleFileOptions};

pub const ARCHIVE_DEFAULT_FILE_EXTENSION: &str = "zip";
pub const ARCHIVE_MIME_TYPE: &str = "application/x-zip";

const CURRENT_DIR_FMT: &str = "%Y-%m-%d_%H_%M_%S";
const PROC_DIR_NAME: &str = "proc";
const LOCK_FILE_NAME: &str = "run.lock";
const REPORT_FILE_NAME: &str = "exit_report.txt";
const KEEP_NUMBER_OF_RUNS: usize = 20;
const READ_CHUNK_SIZE: usize = 65_536;
const PANIC_FILE_EXTENSION: &str = "panic";

static RUN_DIR_HOLDER: OnceLock<PathBuf> = OnceLock::new();
pub type ProcDirArchiveCallback = fn();
static PROC_DIR_ARCHIVE_CREATE_CALLBACK: OnceLock<ProcDirArchiveCallback> = OnceLock::new();

pub fn set_proc_dir(path: PathBuf) {
    RUN_DIR_HOLDER.set(path).expect("Proc directory already set");
    init_proc_directory(RUN_DIR_HOLDER.get().unwrap());
}

pub fn default_proc_dir() -> &'static PathBuf {
    static HOLDER: OnceLock<PathBuf> = OnceLock::new();
    HOLDER.get_or_init(|| crate::misc::get_data_dir().join(std::path::Path::new(PROC_DIR_NAME)))
}

fn write_report_aborted_unexpected(path: &Path) -> Result<()> {
    let report_file_path = path.join(REPORT_FILE_NAME);
    if !report_file_path.try_exists()? {
        std::fs::write(
            report_file_path,
            "The program run was aborted unexpectedly.\n\
            This behavior is typically caused by a SIGKILL, but it can \
            also be the result of a program crash or immediate termination.",
        )?;
    }
    Ok(())
}

fn write_report(content: &str) -> Result<()> {
    let report_file_path = proc_dir().join(REPORT_FILE_NAME);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(report_file_path)
        .with_context(|| "Cannot open report file")?;
    writeln!(file, "{content}").with_context(|| "Cannot write report file")
}

pub fn write_report_success() {
    log::trace!("report success");
    if let Err(err) = write_report("The program run was executed successfully") {
        log::warn!("{err:?}")
    }
    remove_lock_file();
}

pub fn write_report_error(err: &anyhow::Error) {
    log::trace!("report error");
    if let Err(err) = write_report(&format!("The program run exited with error:\n{err:?}")) {
        log::warn!("{err:?}")
    }
    remove_lock_file();
}

#[allow(dead_code)] // clippy warning: field `0` is never read
struct LockFile(File, PathBuf);

impl Drop for LockFile {
    fn drop(&mut self) {
        // LockFile is not removed if the process is unexpected aborted.
        // This behavior is caused by a SIGKILL, crash or immediate termination like std::process::exit().
        let file_name = &self.1;
        _ = std::fs::remove_file(file_name);
    }
}

static LOCK_HOLDER: Mutex<Option<LockFile>> = Mutex::new(None);

fn create_lock_file(path: &Path) -> Result<()> {
    let lock_file_path = path.join(LOCK_FILE_NAME);

    let lock_file = File::create(&lock_file_path)
        .with_context(|| format!("Cannot create file '{}'", lock_file_path.to_string_lossy()))?;
    lock_file
        .try_lock_exclusive()
        .with_context(|| format!("Cannot lock exclusive file '{}'", lock_file_path.to_string_lossy()))?;
    *LOCK_HOLDER.lock().unwrap() = Some(LockFile(lock_file, lock_file_path));
    Ok(())
}

fn remove_lock_file() {
    _ = LOCK_HOLDER.lock().unwrap().take();
}

fn determine_finished_dirs(path: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    if let Ok(entry) = std::fs::read_dir(path) {
        for entry in entry {
            let entry = entry.with_context(|| format!("Cannot list directories in '{}'", path.to_string_lossy()))?;

            let existing_run_dir = entry.path();
            if !existing_run_dir.is_dir() {
                // Skip non-directory entries
                continue;
            }
            let lock_file_path = existing_run_dir.join(LOCK_FILE_NAME);

            match File::open(&lock_file_path)
                .with_context(|| format!("Cannot open lock file '{}'", lock_file_path.to_string_lossy()))
            {
                Ok(lock_file) => match lock_file.try_lock_exclusive() {
                    Ok(locked) => {
                        if locked {
                            // Lock file present - this is an aborted run
                            if let Err(err) = write_report_aborted_unexpected(&existing_run_dir) {
                                log::warn!("{err:?}");
                            }
                            if dir_has_panic(&existing_run_dir)? {
                                // Skip directory, panicked runs should not be processed
                            } else {
                                dirs.push(existing_run_dir);
                            }
                        }
                    }
                    Err(_error) => {
                        // Cannot get lock - skip directory, it is in use
                    }
                },
                Err(error) => match error.downcast_ref::<std::io::Error>() {
                    Some(err) => {
                        if std::io::ErrorKind::NotFound == err.kind() {
                            // No lock file - run is either successful or failed
                            dirs.push(existing_run_dir);
                        } else {
                            return Err(error);
                        }
                    }
                    _ => return Err(error),
                },
            }
        }
    }

    Ok(dirs)
}

fn cleanup_dir(dir: &Path) -> Result<()> {
    let mut dirs = determine_finished_dirs(dir)?;
    if dirs.len() > KEEP_NUMBER_OF_RUNS {
        // Remove oldest runs
        dirs.sort();
        let mut len = dirs.len();
        for dir in dirs.iter() {
            log::trace!("remove old run directory: {dir:?}");
            std::fs::remove_dir_all(dir)
                .with_context(|| format!("Cannot remove directory '{}'", dir.to_string_lossy()))?;
            len -= 1;
            if len <= KEEP_NUMBER_OF_RUNS {
                break;
            }
        }
    }

    Ok(())
}

fn init_proc_directory(data_dir: &Path) {
    std::fs::create_dir_all(data_dir).unwrap_or_else(|error| panic!("Cannot create directory {data_dir:?}: {error:?}"));
    create_lock_file(data_dir).unwrap_or_else(|error| panic!("Cannot lock directory {data_dir:?}: {error:?}"));
}

pub fn proc_dir() -> &'static PathBuf {
    RUN_DIR_HOLDER.get_or_init(|| {
        let default_proc_dir = default_proc_dir();
        cleanup_dir(default_proc_dir).unwrap_or_else(|error| {
            panic!("Cannot cleanup process directories: {error:?}");
        });

        let data_dir = chrono::Local::now().format(CURRENT_DIR_FMT).to_string();
        let data_dir = default_proc_dir.join(std::path::Path::new(&data_dir));
        init_proc_directory(&data_dir);
        data_dir
    })
}

fn create_archive(src_dirs: &[PathBuf], archive_file_path: &Path) -> Result<()> {
    if src_dirs.is_empty() {
        anyhow::bail!("Cannot archive empty list of directories");
    }

    let archive_file = File::create(archive_file_path)
        .with_context(|| format!("Cannot create archive '{}'", archive_file_path.to_string_lossy()))?;

    let mut zip = ZipWriter::new(archive_file);
    let options = SimpleFileOptions::default();

    for src_dir in src_dirs {
        let parent_dir = src_dir
            .parent()
            .unwrap_or_else(|| src_dir)
            .parent()
            .unwrap_or_else(|| src_dir)
            .parent()
            .unwrap_or_else(|| src_dir);
        let walk_dir = WalkDir::new(src_dir);
        let it = walk_dir.into_iter().filter_map(|e| e.ok());
        let mut buffer = [0; READ_CHUNK_SIZE];

        for entry in it {
            let path = entry.path();
            let name = path.strip_prefix(parent_dir).unwrap();

            // Write file or directory explicitly
            // Some unzip tools unzip files with directory paths correctly, some do not!
            if path.is_file() {
                log::trace!("adding file {path:?} as {name:?} ...");
                zip.start_file_from_path(name, options)
                    .with_context(|| format!("Cannot add file '{}' to archive", name.to_string_lossy()))?;
                let mut f = File::open(path).with_context(|| {
                    format!(
                        "Cannot open file '{}' to add it to the archive.",
                        path.to_string_lossy()
                    )
                })?;

                loop {
                    let bytes = f.read(&mut buffer[..]).with_context(|| {
                        format!(
                            "Cannot read from file '{}' to add it to the archive.",
                            path.to_string_lossy()
                        )
                    })?;

                    zip.write_all(&buffer[..bytes]).with_context(|| {
                        format!("Cannot write file buffer '{}' to the archive.", path.to_string_lossy())
                    })?;

                    if bytes < READ_CHUNK_SIZE {
                        break;
                    }
                }
            } else if path.is_dir() && !name.as_os_str().is_empty() {
                // Only if not root! Avoids path spec / warning
                // and mapname conversion failed error on unzip
                log::trace!("adding dir {path:?} as {name:?} ...");
                zip.add_directory(name.to_string_lossy(), options)
                    .with_context(|| format!("Cannot add directory '{}' to the archive", name.to_string_lossy()))?;
            }
        }
    }

    zip.finish()
        .with_context(|| format!("Cannot finish archive '{}'", archive_file_path.to_string_lossy()))?;

    Ok(())
}

fn dir_has_panic(path: &Path) -> Result<bool> {
    if !path.is_dir() {
        return Ok(false);
    }
    let panic_extension = std::ffi::OsString::from(PANIC_FILE_EXTENSION);
    Ok(!std::fs::read_dir(path)?
        .filter(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if !path.is_file() {
                    return false;
                }
                if let Some(extension) = path.extension() {
                    return extension == panic_extension.as_os_str();
                }
            }
            true
        })
        .collect::<std::io::Result<Vec<_>>>()?
        .is_empty())
}

pub fn any_panic() -> Result<bool> {
    Ok(std::fs::read_dir(default_proc_dir())?
        .map(|entry| match entry {
            Ok(entry) => dir_has_panic(entry.path().as_path()),
            Err(err) => Err(err.into()),
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .any(|item| *item))
}

#[cfg(feature = "problem_report_dialog")]
pub(crate) fn create_problem_report_file_name(binary_name: &str) -> String {
    format!("{binary_name}_problem_report.{ARCHIVE_DEFAULT_FILE_EXTENSION}")
}

pub fn move_panics_to_trash() -> Result<()> {
    let directories = std::fs::read_dir(default_proc_dir())?
        .map(|entry| Ok(entry?.path()))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter_map(|dir| match dir_has_panic(&dir) {
            Ok(has_panic) => {
                if has_panic {
                    Some(Ok(dir))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<_>>>()?;
    trash::delete_all(directories).with_context(|| "Cannot move failed executions to trash")?;
    Ok(())
}

#[cfg(feature = "create_report_dialog")]
pub(crate) fn create_report_file_name(binary_name: &str) -> String {
    format!("{binary_name}_report.{ARCHIVE_DEFAULT_FILE_EXTENSION}")
}

pub fn proc_dir_archive_set_callback(callback: ProcDirArchiveCallback) {
    PROC_DIR_ARCHIVE_CREATE_CALLBACK.set(callback).unwrap();
}

pub fn archive_and_remove_panics(archive_file_path: &Path) -> Result<()> {
    if let Some(callback) = PROC_DIR_ARCHIVE_CREATE_CALLBACK.get() {
        callback();
    }
    let directories = std::fs::read_dir(default_proc_dir())?
        .map(|entry| Ok(entry?.path()))
        .collect::<Result<Vec<_>>>()?;
    create_archive(&directories, archive_file_path)?;
    for dir in directories {
        if dir_has_panic(&dir)? {
            log::trace!("remove old directory: {dir:?}");
            std::fs::remove_dir_all(&dir)
                .with_context(|| format!("Cannot remove directory '{}'", dir.to_string_lossy()))?;
        }
    }
    Ok(())
}

pub fn setup_panic() {
    panic::set_hook(Box::new({
        let log_dir = proc_dir().to_owned();
        move |info| {
            let backtrace = std::backtrace::Backtrace::force_capture();
            let thread = std::thread::current();
            let thread_name = thread.name().unwrap_or("<unnamed>");
            let cause = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };

            let dump = match info.location() {
                Some(location) => {
                    format!(
                        "Thread '{thread_name}' panicked at '{cause}': {file_name}:{line}:{column}\n{backtrace}",
                        file_name = location.file(),
                        line = location.line(),
                        column = location.column()
                    )
                }
                None => format!("Thread '{thread_name}' panicked at '{cause}'\n{backtrace}"),
            };
            std::eprintln!("{dump}");
            let file_name = format!(
                "{}.{}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                PANIC_FILE_EXTENSION
            );
            let panic_file = log_dir.join(file_name);
            if let Err(err) = std::fs::write(&panic_file, dump) {
                std::eprintln!(
                    "Cannot write panic into file '{}': {:?}",
                    panic_file.to_string_lossy(),
                    err
                );
            }
        }
    }));
}

use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use std::{fs::File, io::Write, path::PathBuf};

#[allow(dead_code)]
pub(crate) const SUPPORT_EMAIL: &str = "support@x-software.com";

static PROJECT_DATA_DIR: OnceCell<PathBuf> = OnceCell::new();

pub fn init(project_data_dir: PathBuf) {
    PROJECT_DATA_DIR.set(project_data_dir).expect("Already initialized");
    crate::localization::init();
}

#[cfg(any(feature = "create_report_dialog", feature = "problem_report_dialog"))]
pub fn init_gui() -> anyhow::Result<()> {
    mxl_relm4_components::init()?;
    relm4_icons::initialize_icons();
    Ok(())
}

pub(crate) fn get_data_dir() -> &'static PathBuf {
    PROJECT_DATA_DIR.get().expect("Need to be initialized")
}

#[cfg(feature = "with_test")]
pub fn init_test() {
    use once_cell::sync::Lazy;
    use tempfile::TempDir;

    static TMP_DIR: Lazy<TempDir> = Lazy::new(|| TempDir::new().expect("Failed create tmp directory"));

    init(TMP_DIR.path().to_path_buf());
}

pub fn log_sysinfo(level: log::Level) {
    #[cfg(feature = "sysinfo")]
    {
        use sysinfo::System;

        log::log!(
            level,
            "System='{}' Kernel='{}' OS='{}' Hostname={}",
            System::name().unwrap_or("unknown".into()),
            System::kernel_version().unwrap_or("unknown".into()),
            System::long_os_version().unwrap_or("unknown".into()),
            System::host_name().unwrap_or("unknown".into())
        );
    }
    #[cfg(not(feature = "sysinfo"))]
    {
        _ = level
    }
}

pub fn create_sysinfo_dump() {
    #[cfg(feature = "sysinfo")]
    {
        fn create_sysinfo() -> Result<()> {
            use sysinfo::{Components, Disks, Networks, System};

            let sysinfo_file_path = crate::proc_dir::proc_dir().join("sysinfo.txt");
            let mut file = File::options()
                .create(true)
                .append(true)
                .open(&sysinfo_file_path)
                .with_context(|| format!("Cannot create file '{}'", sysinfo_file_path.to_string_lossy()))?;

            let mut sys = sysinfo::System::new_all();
            sys.refresh_all();

            let mut out = Vec::new();
            writeln!(&mut out, "=> system:")?;
            // RAM and swap information:
            writeln!(&mut out, "total memory: {} bytes", sys.total_memory())?;
            writeln!(&mut out, "used memory : {} bytes", sys.used_memory())?;
            writeln!(&mut out, "total swap  : {} bytes", sys.total_swap())?;
            writeln!(&mut out, "used swap   : {} bytes", sys.used_swap())?;

            // Display system information:
            writeln!(&mut out, "System name:             {:?}", System::name())?;
            writeln!(&mut out, "System kernel version:   {:?}", System::kernel_version())?;
            writeln!(&mut out, "System OS version:       {:?}", System::os_version())?;
            writeln!(&mut out, "System long OS version:  {:?}", System::long_os_version())?;
            writeln!(&mut out, "System host name:        {:?}", System::host_name())?;

            // Number of CPUs:
            writeln!(&mut out, "NB CPUs: {}", sys.cpus().len())?;

            // Display processes ID, name na disk usage:
            if false {
                for (pid, process) in sys.processes() {
                    writeln!(&mut out, "[{pid}] {:?} {:?}", process.name(), process.disk_usage())?;
                }
            }

            // We display all disks' information:
            writeln!(&mut out, "=> disks:")?;
            let disks = Disks::new_with_refreshed_list();
            for disk in &disks {
                writeln!(&mut out, "{disk:?}")?;
            }

            // Network interfaces name, total data received and total data transmitted:
            let networks = Networks::new_with_refreshed_list();
            writeln!(&mut out, "=> networks:")?;
            for (interface_name, data) in &networks {
                writeln!(
                    &mut out,
                    "{interface_name}: {} B (down) / {} B (up)",
                    data.total_received(),
                    data.total_transmitted(),
                )?;
                // If you want the amount of data received/transmitted since last call
                // to `Networks::refresh`, use `received`/`transmitted`.
            }

            // Components temperature:
            let components = Components::new_with_refreshed_list();
            writeln!(&mut out, "=> components:")?;
            for component in &components {
                writeln!(&mut out, "{component:?}")?;
            }

            file.write_all(out.as_slice())?;
            Ok(())
        }

        if let Err(err) = create_sysinfo() {
            log::warn!("Cannot create system information: {:?}", err);
        }
    }
}

pub fn exec_cmd_and_dump_pipes(command: std::process::Command) {
    fn exec_cmd(command: std::process::Command) -> Result<()> {
        let path = crate::proc_dir::proc_dir();
        let mut stdout_file_name = std::ffi::OsString::new();
        stdout_file_name.push(command.get_program());
        stdout_file_name.push("_stdout.txt");
        let mut stdout_file = File::options()
            .create(true)
            .append(true)
            .open(path.join(stdout_file_name))?;
        writeln!(&mut stdout_file, "{command:?}")?;
        let mut stderr_file_name = std::ffi::OsString::new();
        stderr_file_name.push(command.get_program());
        stderr_file_name.push("_stderr.txt");
        let mut stderr_file = File::options()
            .create(true)
            .append(true)
            .open(path.join(stderr_file_name))?;
        writeln!(&mut stderr_file, "{command:?}")?;
        let mut command = command;
        command.stdout(stdout_file).stderr(stderr_file);
        command
            .spawn()
            .with_context(|| format!("Cannot start {:?}", command.get_program()))?
            .wait()
            .with_context(|| format!("Cannot wait for {:?}", command.get_program()))?;
        Ok(())
    }

    if let Err(err) = exec_cmd(command) {
        log::warn!("Cannot execute command: {:?}", err);
    }
}

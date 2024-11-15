use anyhow::{Context, Result};
use log::*;
use mxl_relm4_components::relm4::{gtk::gio, prelude::*};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

mod player;

use player::{
    about::APP_ID,
    app::{App, AppInit},
    init::init,
};

#[test]
fn playback() -> Result<()> {
    init()?;

    let mut uris = vec![];

    if let Some(parent) = PathBuf::from(file!()).parent() {
        if let Some(name) = parent.file_name() {
            let data_path = PathBuf::from(name).join("data");
            for file in data_path
                .read_dir()
                .with_context(|| format!("Cannot read {:?} dir", data_path))?
            {
                match file {
                    Ok(file) => uris.push(file.path()),
                    Err(error) => error!("Cannot list file - {error:?}"),
                }
            }
        }
    }

    let error_channel = Arc::new(Mutex::new(None));

    let adw_app = adw::Application::new(Some(APP_ID), gio::ApplicationFlags::default());
    let app = RelmApp::from_app(adw_app);
    app.with_args(vec![]).run::<App>(AppInit {
        uris,
        quit_on_stopped: true,
        error_channel: Arc::clone(&error_channel),
    });

    let mut error = error_channel.lock().unwrap();
    if let Some(error) = error.take() {
        return Err(error);
    }

    Ok(())
}

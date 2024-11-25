use anyhow::{Context, Result};
use log::*;
use mxl_relm4_components::relm4::{gtk::gio, prelude::*};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

mod player;

use player::{
    about::APP_ID,
    app::{App, AppInit, AppMsg},
    init::init,
    ControllerFeedback,
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
        error_channel: Arc::clone(&error_channel),
        test_controller: |recv, sender| {
            debug!("Test controller launched...");
            loop {
                match recv.recv_timeout(Duration::from_secs(10)) {
                    Err(error) => {
                        sender.input(AppMsg::TestError(anyhow::anyhow!(
                            "Failed to receive from receiver channel: {:?}",
                            error
                        )));
                        break;
                    }
                    Ok(feedback) => match feedback {
                        ControllerFeedback::AppStateChanged(state) => {
                            debug!("App state changed: {:?}", state);
                        }
                        ControllerFeedback::PlayerMediaInfoUpdated(info) => {
                            debug!("Media info updated: {:?}", info);
                        }
                        ControllerFeedback::PlayerInitialized => {
                            debug!("Player initialized");
                            sender.input(AppMsg::TogglePlayPause);
                        }
                        ControllerFeedback::PlayerDurationChanged(duration) => {
                            debug!("Duration changed: {:?}", duration);
                        }
                        ControllerFeedback::PlayerPositionUpdated(pos) => {
                            debug!("Position changed: {:?}", pos);
                            match pos {
                                0.0..2.0 => sender.input(AppMsg::Seek(7.0)),
                                3.0..6.0 => sender.input(AppMsg::TestError(anyhow::anyhow!(
                                    "Not expecting playback in region 3-6 seconds, seeking to 7 seconds"
                                ))),
                                _ => {}
                            }
                        }
                        ControllerFeedback::PlayerSeekDone => debug!("Seek done"),
                        ControllerFeedback::PlayerEndOfStream(uri) => {
                            debug!("End of stream of uri: {:?}", uri);
                        }
                        ControllerFeedback::PlaylistChanged(change) => {
                            debug!("Playlist changed: {:?}", change);
                        }
                        ControllerFeedback::PlaylistSwitchUri(uri) => {
                            debug!("Playlist switched to uri: {:?}", uri);
                        }
                        ControllerFeedback::PlaylistEndOfPlaylist => {
                            debug!("End of playlist - quit app");
                            sender.input(AppMsg::Quit);
                            break;
                        }
                        msg => {
                            sender.input(AppMsg::TestError(anyhow::anyhow!(
                                "Unexpected controller feedback from App: {:?}",
                                msg
                            )));
                        }
                    },
                }
            }
        },
    });

    let mut error = error_channel.lock().unwrap();
    if let Some(error) = error.take() {
        return Err(error);
    }

    Ok(())
}

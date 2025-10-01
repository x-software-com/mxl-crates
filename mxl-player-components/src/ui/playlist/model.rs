use gst_pbutils::DiscovererInfo;
use log::*;
use mxl_relm4_components::relm4::{
    Sender, adw::prelude::*, factory::FactoryVecDeque, gtk::gdk::DragAction, prelude::*,
};
use std::path::PathBuf;

use crate::ui::playlist::messages::{
    PlaylistChange, PlaylistCommandOutput, PlaylistComponentInput, PlaylistComponentOutput, PlaylistState, RepeatMode,
    SortOrder,
};
use crate::uri_helpers::uri_from_pathbuf;

use super::factory::PlaylistEntryInit;
pub use super::factory::PlaylistEntryModel;

#[derive(Debug, Clone)]
pub struct PlaylistEntry {
    pub uri: PathBuf,
    pub media_info: Option<DiscovererInfo>,
}

impl From<PathBuf> for PlaylistEntry {
    fn from(uri: PathBuf) -> Self {
        PlaylistEntry {
            uri: uri.clone(),
            media_info: None,
        }
    }
}

#[derive(Debug)]
pub struct PlaylistComponentInit {
    pub uris: Vec<PlaylistEntry>,
    pub mark_index_as_playing: Option<usize>,
    pub repeat: RepeatMode,
    pub is_user_mutable: bool,
    pub show_file_index: bool,
}

impl Default for PlaylistComponentInit {
    fn default() -> Self {
        Self {
            uris: vec![],
            mark_index_as_playing: None,
            repeat: RepeatMode::default(),
            is_user_mutable: true,
            show_file_index: false,
        }
    }
}

pub struct PlaylistComponentModel {
    pub uris: FactoryVecDeque<PlaylistEntryModel>,
    pub index: Option<DynamicIndex>,
    pub state: PlaylistState,
    pub show_file_index: bool,
    pub show_placeholder: bool,
    pub repeat: RepeatMode,
    pub thread_pool: Option<rusty_pool::ThreadPool>,
    pub is_user_mutable: bool,
}

#[allow(dead_code)]
pub(super) enum InsertMode {
    Front,
    AtIndex(DynamicIndex),
    Back,
}

impl PlaylistComponentModel {
    pub(super) fn init_thread_pool() -> rusty_pool::ThreadPool {
        const DEFAULT_NUMBER_THREADS: usize = 3;
        let thread_count = std::thread::available_parallelism()
            .map_or(DEFAULT_NUMBER_THREADS, |v| v.get().min(DEFAULT_NUMBER_THREADS));
        debug!("Create thread pool with {thread_count} number of threads");
        rusty_pool::Builder::new()
            .name("mxl_playlist_pool".to_owned())
            .max_size(thread_count)
            .build()
    }

    pub fn dynamic_index(&self) -> Option<&DynamicIndex> {
        self.index.as_ref()
    }

    pub fn new_drop_target(sender: Sender<PlaylistComponentInput>) -> gtk::DropTarget {
        let formats = gtk::gdk::ContentFormatsBuilder::new()
            .add_type(gtk::gdk::FileList::static_type())
            .add_type(gtk::gio::File::static_type())
            .build();
        let drop_target = gtk::DropTarget::builder()
            .actions(DragAction::COPY)
            .formats(&formats)
            .build();

        drop_target.set_types(&[gtk::gdk::FileList::static_type(), gtk::gio::File::static_type()]);

        drop_target.connect_drop(move |_, value, _, _| {
            if let Ok(files) = value.get::<gtk::gdk::FileList>() {
                let files: Vec<_> = files.files().iter().filter_map(|file| file.path()).collect();

                sender.emit(PlaylistComponentInput::Add(files));
                return true;
            } else if let Ok(file) = value.get::<gtk::gio::File>() {
                if let Some(file) = file.path() {
                    sender.emit(PlaylistComponentInput::Add(vec![file]));
                    return true;
                }
            }
            false
        });

        drop_target
    }

    pub(super) fn add_uris(
        &mut self,
        sender: &ComponentSender<Self>,
        insert_mode: InsertMode,
        uris: &Vec<PlaylistEntry>,
    ) {
        macro_rules! insert {
            ($edit:expr, $insert_mode:expr, $entry:expr) => {
                match &$insert_mode {
                    InsertMode::Front => {
                        $edit.push_front($entry);
                    }
                    InsertMode::AtIndex(index) => {
                        $edit.insert(index.current_index(), $entry);
                    }
                    InsertMode::Back => {
                        $edit.push_back($entry);
                    }
                }
            };
        }

        let mut edit = self.uris.guard();
        for entry in uris {
            match uri_from_pathbuf(&entry.uri) {
                Ok(file) => {
                    let file_name = entry
                        .uri
                        .file_name()
                        .map(|x| x.to_str().unwrap_or_default().to_string());

                    insert!(
                        edit,
                        insert_mode,
                        PlaylistEntryInit {
                            uri: file,
                            short_uri: file_name,
                            media_info: entry.media_info.clone(),
                            error: None,
                            show_index: self.show_file_index,
                            movable: self.is_user_mutable,
                            removable: self.is_user_mutable,
                            drop_files_to_add: self.is_user_mutable,
                        }
                    );
                }
                Err(error) => {
                    let file_name = entry
                        .uri
                        .file_name()
                        .map(|x| x.to_str().unwrap_or_default().to_string());
                    let file = entry.uri.to_str().unwrap_or_default().to_string();

                    insert!(
                        edit,
                        insert_mode,
                        PlaylistEntryInit {
                            uri: file,
                            short_uri: file_name,
                            media_info: entry.media_info.clone(),
                            error: Some(error),
                            show_index: self.show_file_index,
                            movable: self.is_user_mutable,
                            removable: self.is_user_mutable,
                            drop_files_to_add: self.is_user_mutable,
                        }
                    );
                }
            }
        }
        sender
            .command_sender()
            .emit(PlaylistCommandOutput::ShowPlaceholder(edit.is_empty()));
        sender
            .output_sender()
            .emit(PlaylistComponentOutput::PlaylistChanged(PlaylistChange::Added));
        drop(edit);
    }

    pub(super) fn sort_factory(&mut self, order: &SortOrder) {
        macro_rules! sort_factory {
            ($guard:expr, $key:ident) => {{
                let length = $guard.len();
                for from_pos in 1..length {
                    let mut j = from_pos;
                    while j > 0 && $guard.get(j).unwrap().$key < $guard.get(j - 1).unwrap().$key {
                        trace!(
                            "Swap item {}[{:?}] with item {}[{:?}]",
                            j,
                            $guard.get(j).unwrap().$key,
                            j - 1,
                            $guard.get(j - 1).unwrap().$key
                        );
                        $guard.swap(j, j - 1);
                        j -= 1;
                    }
                }
            }};
        }

        let mut guard = self.uris.guard();
        if !guard.is_empty() {
            match order {
                SortOrder::StartTime => {
                    sort_factory!(guard, date_time);
                }
                SortOrder::ShortUri => {
                    sort_factory!(guard, short_uri);
                }
            }
        }
    }

    pub(super) fn previous(&mut self, sender: &ComponentSender<Self>) {
        if let Some(index) = self.index.as_ref() {
            if let Some(i) = index.current_index().checked_sub(1) {
                if let Some(entry) = self.uris.guard().get(i) {
                    // Switch to previous file:
                    debug!("Playlist previous -> switch to index {i}");
                    sender.input(PlaylistComponentInput::Switch(entry.index.clone()));
                    return;
                }
            }
            if let Some(entry) = self.uris.guard().get(index.current_index()) {
                // Restart playback of current first file in playlist:
                debug!("Playlist previous -> switch to index {}", index.current_index());
                sender.input(PlaylistComponentInput::Switch(entry.index.clone()));
            }
        }
    }

    pub(super) fn next(&mut self, sender: &ComponentSender<Self>) {
        if let Some(current_index) = self.index.as_ref() {
            if let Some(i) = current_index.current_index().checked_add(1) {
                if let Some(entry) = self.uris.guard().get(i) {
                    // Switch to next file:
                    debug!("Playlist next -> switch to index {i}");
                    sender.input(PlaylistComponentInput::Switch(entry.index.clone()));
                    return;
                }
            }
            match self.repeat {
                RepeatMode::Off => (),
                RepeatMode::All => {
                    if let Some(entry) = self.uris.guard().get(0) {
                        debug!("Playlist repeat all -> switch to index 0");
                        sender.input(PlaylistComponentInput::Switch(entry.index.clone()));
                        return;
                    }
                }
            }
            sender.input(PlaylistComponentInput::EndOfPlaylist(current_index.clone()));
        }
    }
}

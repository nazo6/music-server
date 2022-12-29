use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use lofty::{read_from_path, Accessor, AudioFile};
use tokio::task::{spawn, spawn_blocking};
use walkdir::WalkDir;

use server_core::library::get_library;

use crate::{errors::Error, BackgroundEvent, Notifier};

struct ScanResult {
    pub tracks: Vec<TrackInfo>,
    pub artists: Vec<String>,
    pub albums: Vec<String>,
}

struct TrackInfo {
    file_name: String,
    duration: Duration,
    tag: Tag,
}

struct Tag {
    title: Option<String>,
    album: Option<String>,
    artist: Option<String>,
    genre: Option<String>,
}

pub async fn scan(library_id: i32, notifier: Notifier) -> Result<(), Error> {
    let library = get_library(library_id).await?;
    let library = library.ok_or_else(|| Error::GeneralError("Invalid library id".to_string()))?;

    let mut tracks = Vec::new();

    let count = Arc::new(Mutex::new(0));
    let walkdir = WalkDir::new(library.path).follow_links(true);
    for entry in walkdir.into_iter().flatten() {
        if entry.path().is_file() {
            let res = {
                let count = Arc::clone(&count);
                spawn_blocking(move || {
                    let info = extract_info_from_path(entry.path());
                    match info {
                        Ok(info) => {
                            let mut count = count.lock().unwrap();
                            *count += 1;
                            Ok(info)
                        }
                        Err(err) => Err(err),
                    }
                })
                .await
                .unwrap()
            };

            if let Ok(info) = res {
                let count = *count.lock().unwrap();
                tracks.push(info);
                notifier
                    .send(BackgroundEvent::UpdateScan {
                        scanning: true,
                        count,
                        library_id,
                    })
                    .await
                    .unwrap();
            }
        }
    }

    Ok(())
}

fn extract_info_from_path(path: &std::path::Path) -> Result<TrackInfo, Error> {
    let tagged_file = read_from_path(path)?;
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let properties = tagged_file.properties();
    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => Some(primary_tag),
        None => tagged_file.first_tag(),
    };

    let tag = match tag {
        Some(tag) => Tag {
            title: tag.title().map(|s| s.to_string()),
            album: tag.album().map(|s| s.to_string()),
            artist: tag.artist().map(|s| s.to_string()),
            genre: tag.genre().map(|s| s.to_string()),
        },
        None => Tag {
            title: None,
            album: None,
            artist: None,
            genre: None,
        },
    };

    Ok(TrackInfo {
        duration: properties.duration(),
        file_name,
        tag,
    })
}

use std::time::Duration;

use lofty::{read_from_path, Accessor, AudioFile};
use tokio::task::{spawn, spawn_blocking};
use walkdir::WalkDir;

use server_core::library::get_library;

use crate::{errors::Error, BackgroundEvent, Notifier};

struct ScanResult {
    pub tracks: String,
}

struct FileInfo {
    duration: Duration,
    file_name: String,
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

    let res = spawn(async move {
        let mut count = 0;
        for entry in WalkDir::new(library.path)
            .follow_links(true)
            .into_iter()
            .flatten()
        {
            if entry.path().is_file() {
                spawn_blocking(move || {
                    let info = extract_info_from_path(entry.path());
                    match info {
                        Ok(info) => {
                            count += 1;
                            Ok(())
                        }
                        Err(err) => Err(err),
                    }
                });
            }
        }

        notifier
            .send(BackgroundEvent::UpdateScan {
                scanning: true,
                count,
                library_id,
            })
            .await
            .unwrap();
    })
    .await;

    Ok(())
}

fn extract_info_from_path(path: &std::path::Path) -> Result<FileInfo, Error> {
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

    Ok(FileInfo {
        duration: properties.duration(),
        file_name,
        tag,
    })
}

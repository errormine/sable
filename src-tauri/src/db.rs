use std::{error::Error, path::{Path, PathBuf}, thread};

use audiotags::Tag;
use jwalk::WalkDir;
use rusqlite::{params, Connection};
use serde_json::json;
use tauri::Manager;

use crate::audio;

fn is_audio_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.extension().unwrap().to_str().unwrap() {
        "mp3" | "flac" | "wav" | "ogg" | "aac" => true,
        _ => false
    }
}

fn get_song_count(dir: &Path) -> i32 {
    let mut count = 0;

    for entry in WalkDir::new(dir).sort(true) {
        if is_audio_file(&entry.unwrap().path()) {
            count += 1;
        }
    }

    return count;
}

#[derive(Debug)]
struct SongMetadata {
    file_path: String,
    cover_path: Option<String>,
    title: String,
    artist: String,
    album_title: String,
    album_artist: String,
    track_number: u16,
    disc_number: u16,
    duration: u64,
    year: i32,
    genre: String
}

impl Clone for SongMetadata {
    fn clone(&self) -> Self {
        SongMetadata {
            file_path: self.file_path.clone(),
            cover_path: self.cover_path.clone(),
            title: self.title.clone(),
            artist: self.artist.clone(),
            album_title: self.album_title.clone(),
            album_artist: self.album_artist.clone(),
            track_number: self.track_number,
            disc_number: self.disc_number,
            duration: self.duration,
            year: self.year,
            genre: self.genre.clone()
        }
    }
}

fn tag_to_string(option: Option<&str>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => String::from("Unknown"),
    }
}

fn get_metadata(song: &PathBuf) -> Result<SongMetadata, Box<dyn Error>> {
    let tag = Tag::new().read_from_path(&song)?;
    
    let file_path = song.clone().into_os_string().into_string().unwrap();
    let mut cover_path = None;
    let cover_names = ["cover.jpg", "cover.png", "folder.jpg", "folder.png", "front.jpg", "front.png"];
    for entry in WalkDir::new(song.parent().unwrap()) {
        let path = entry.unwrap().path();

        if path.is_file() {
            let lowercase = path.file_name().unwrap().to_ascii_lowercase();
            let name = lowercase.to_str().unwrap();
            
            if cover_names.contains(&name) {
                cover_path = Some(path.into_os_string().into_string().unwrap());
                break;
            };
        };
    };

    let title = tag_to_string(tag.title());
    let artist = tag_to_string(tag.artist());
    let album_title = tag_to_string(tag.album_title());
    let album_artist = match tag.album_artist() {
        Some(album_artist) => String::from(album_artist),
        None => String::from(&artist)
    };

    let track_number = tag.track_number().unwrap_or(0);
    let disc_number = tag.disc_number().unwrap_or(0);
    let duration = match tag.duration() {
        Some(d) => d as u64,
        None => audio::get_duration(&file_path)
    };

    let year = tag.year().unwrap_or(0);
    let genre = tag_to_string(tag.genre());

    return Ok(SongMetadata {
        file_path,
        cover_path,
        title,
        artist,
        album_title,
        album_artist,
        track_number,
        disc_number,
        duration,
        year,
        genre
    });
}

fn commit_to_db(songs: Vec<SongMetadata>) -> Result<(), Box<dyn Error>> {
    let mut db = Connection::open("D:/Documents/music.db")?;
    let tx = db.transaction()?;

    for song in songs {
        tx.execute(
            "INSERT OR REPLACE INTO album (cover_path, title, artist, year, genre) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &song.cover_path,
                &song.album_title,
                &song.album_artist,
                &song.year,
                &song.genre,
            ]
        )?;

        tx.execute(
            "INSERT OR REPLACE INTO song (file_path, cover_path, title, artist, album_title, album_artist, track_number, disc_number, duration, year, genre)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                &song.file_path,
                &song.cover_path,
                &song.title,
                &song.artist,
                &song.album_title,
                &song.album_artist,
                &song.track_number,
                &song.disc_number,
                &song.duration,
                &song.year,
                &song.genre,
            ]
        )?;
    };

    tx.commit()?;
    return Ok(());
}

#[tauri::command]
pub fn register_file(file_path: &Path, app: tauri::AppHandle) {
    let file_path = file_path.to_path_buf();
    
    thread::spawn(move || {
        let metadata = get_metadata(&file_path);

        match metadata {
            Ok(metadata) => {
                let songs_metadata = vec![metadata];
                commit_to_db(songs_metadata).unwrap();
                let result = json!({
                    "message": "File updated successfully",
                    "type": "success",
                    "dismissable": true,
                    "timeout": 3000
                }).to_string();
                app.emit_all("register_file_finished", crate::Payload { message: result }).unwrap();
            },
            Err(e) => {
                let result = json!({
                    "message": e.to_string(),
                    "type": "error",
                    "dismissable": true,
                    "timeout": 5000
                }).to_string();
                app.emit_all("register_file_finished", crate::Payload { message: result }).unwrap();
            }
        };
    });
}

#[tauri::command]
pub fn register_dir(dir: &Path, app: tauri::AppHandle) {
    let dir = dir.to_path_buf();
    
    thread::spawn(move || {
        let songs = get_song_count(&dir);
        app.emit_all("total_songs", crate::Payload { message: songs.to_string() }).unwrap();

        let mut songs_metadata = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        for entry in WalkDir::new(&dir).sort(true) {
            let song = entry.unwrap().path();

            if !is_audio_file(&song) {
                continue;
            }

            if let Ok(metadata) = get_metadata(&song) {
                songs_metadata.push(metadata);
                successful += 1;
                app.emit_all("songs_registered", crate::Payload { message: successful.to_string() }).unwrap();
            } else {
                failed += 1;
                // TODO: keep track of which files failed to read
            };
        };

        if let Err(e) = commit_to_db(songs_metadata) {
            let result = json!({
                "message": e.to_string(),
                "type": "error",
                "dismissable": true,
                "timeout": 5000
            }).to_string();
            app.emit_all("register_songs_finished", crate::Payload { message: result }).unwrap();
        };


        let mut message = format!("Registered {} songs", successful);
        if failed > 0 {
            message += format!(", {} could not be read", failed).as_str();
        }
        let result = json!({
            "message": message,
            "type": "success",
            "dismissable": true,
            "timeout": 5000
        }).to_string();

        app.emit_all("register_songs_finished", crate::Payload { message: result }).unwrap();
    });
}

#[tauri::command]
pub fn get_all_albums() -> String {
    let db = Connection::open("D:/Documents/music.db");

    match db {
        Ok(db) => {
            let mut stmt = db.prepare("SELECT * FROM album ORDER BY artist, title").unwrap();
            let mut rows = stmt.query(params![]).unwrap();
            
            let mut albums_json = Vec::new();
            while let Some(row) = rows.next().unwrap() {
                let cover_path: String = row.get(0).unwrap_or_default();
                let title: String = row.get(1).unwrap();
                let artist: String = row.get(2).unwrap();
                let year: u32 = row.get(3).unwrap_or(0);
                let genre: String = row.get(4).unwrap();

                let album = json!({
                    "cover_path": cover_path,
                    "title": title,
                    "artist": artist,
                    "year": year,
                    "genre": genre,
                });

                albums_json.push(album);
            }

            return json!(albums_json).to_string();
        },
        Err(_) => return String::from("[]")
    };
}

#[tauri::command]
pub fn get_albums_by_artist(artist: String) -> String {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let mut stmt = db.prepare("SELECT * FROM album WHERE artist = ?1 ORDER BY title").unwrap();
    let mut rows = stmt.query(params![artist]).unwrap();
    
    let mut albums_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let cover_path: String = row.get(0).unwrap_or_default();
        let title: String = row.get(1).unwrap();
        let artist: String = row.get(2).unwrap();
        let year: u32 = row.get(3).unwrap_or(0);
        let genre: String = row.get(4).unwrap();

        let album = json!({
            "cover_path": cover_path,
            "title": title,
            "artist": artist,
            "year": year,
            "genre": genre,
        });

        albums_json.push(album);
    }

    return json!(albums_json).to_string();
}

#[tauri::command]
pub fn get_songs_by_album(title: String, artist: String) -> String {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let mut stmt = db.prepare("SELECT * FROM song WHERE album_title = ?1 AND album_artist = ?2 ORDER BY disc_number, track_number").unwrap();
    let mut rows = stmt.query(params![title, artist]).unwrap();
    
    let mut songs_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let file_path: String = row.get(0).unwrap();
        let cover_path: String = row.get(1).unwrap_or_default();
        let title: String = row.get(2).unwrap();
        let artist: String = row.get(3).unwrap();
        let album_title: String = row.get(4).unwrap();
        let album_artist: String = row.get(5).unwrap();
        let track_number: u32 = row.get(6).unwrap_or(0);
        let disc_number: u32 = row.get(7).unwrap_or(0);
        let duration: u32 = row.get(8).unwrap_or(0);
        let year: u32 = row.get(9).unwrap_or(0);
        let genre: String = row.get(10).unwrap();

        let song = json!({
            "file_path": file_path,
            "cover_path": cover_path,
            "title": title,
            "artist": artist,
            "album_title": album_title,
            "album_artist": album_artist,
            "track_number": track_number,
            "disc_number": disc_number,
            "duration": duration,
            "year": year,
            "genre": genre,
        });

        songs_json.push(song);
    }

    return json!(songs_json).to_string();
}

#[tauri::command]
pub fn get_all_artists() -> String {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let mut stmt = db.prepare("SELECT DISTINCT artist FROM album ORDER BY artist").unwrap();
    let mut rows = stmt.query(params![]).unwrap();
    
    let mut artists_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let artist: String = row.get(0).unwrap();
        let album_count: u16 = db.query_row("SELECT COUNT(*) FROM album WHERE artist = ?1", params![artist], |row| row.get(0)).unwrap();

        let artist = json!({
            "name": artist,
            "album_count": album_count,
        });
        
        artists_json.push(artist);
    }

    return json!(artists_json).to_string();
}

#[tauri::command]
pub fn remove_album(album: String, artist: String) {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    db.execute("DELETE FROM song WHERE album_title = ?1 AND album_artist = ?2", params![album, artist]).unwrap();
    db.execute("DELETE FROM album WHERE title = ?1 AND artist = ?2", params![album, artist]).unwrap();
}

#[tauri::command]
pub fn remove_song(file_path: String) {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    db.execute("DELETE FROM song WHERE file_path = ?1", params![file_path]).unwrap();
}

#[tauri::command]
pub fn update_metadata_song(
    file_path: String,
    cover_path: String,
    title: String,
    artist: String,
    album_title: String,
    album_artist: String,
    track_number: u16,
    disc_number: u16,
    year: i32,
    genre: String
) -> String {
    let tag = Tag::new().read_from_path(&file_path);

    match tag { 
        Ok(mut tag) => {
            tag.set_title(&title);
            tag.set_artist(&artist);
            tag.set_album_title(&album_title);
            tag.set_album_artist(&album_artist);
            tag.set_track_number(track_number);
            tag.set_disc_number(disc_number);
            tag.set_year(year);
            tag.set_genre(&genre);
        
            match tag.write_to_path(&file_path) {
                Ok(_) => {},
                Err(e) => {
                    return String::from(e.to_string());
                }
            };
        },
        Err(e) => {
            return String::from(e.to_string());
        }
    }

    println!("cover_path: {}", cover_path);

    if cover_path != "" {
        // TODO: copy cover to song directory
    }
    
    remove_song(file_path);
    return String::from("success");
}

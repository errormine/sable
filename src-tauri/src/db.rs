use std::{error::Error, path::{Path, PathBuf}, thread, time::Instant};

use audiotags::Tag;
use jwalk::WalkDir;
use rusqlite::{params, Connection};
use serde_json::json;
use tauri::Manager;

use crate::audio;

fn get_song_files(dir: &Path) -> Vec<PathBuf> {
    let mut songs: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(dir).sort(true) {
        let path = entry.unwrap().path();

        if path.is_file() {
            let extension = path.extension().unwrap();
            
            match extension.to_str().unwrap() {
                "mp3" => songs.push(path.to_path_buf()),
                "flac" => songs.push(path.to_path_buf()),
                _ => continue
            }
        }
    }

    return songs;
}

#[derive(Debug)]
struct SongMetadata {
    file_path: String,
    cover_path: String,
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
    let cover_path = song.parent().unwrap().join("Cover.jpg").into_os_string().into_string().unwrap();
    let title = tag_to_string(tag.title());
    let artist = tag_to_string(tag.artist());
    let album_title = tag_to_string(tag.album_title());
    let album_artist = tag_to_string(tag.album_artist());
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

fn get_metadata_vec(songs: &Vec<PathBuf>, app: tauri::AppHandle) -> Result<Vec<SongMetadata>, Box<dyn Error>>{
    let mut songs_metadata = Vec::new();
    let mut progress = 0;

    for song in songs {
        match Tag::new().read_from_path(&song) {
            Ok(_tag) => {
                let song_metadata = get_metadata(&song)?;
                
                songs_metadata.push(song_metadata);
                progress += 1;
                app.emit_all("songs_registered", crate::Payload { message: progress.to_string() }).unwrap();
            },
            Err(_) => continue
        }
    }

    return Ok(songs_metadata);
}

#[tauri::command]
pub fn register_dir(dir: &Path, app: tauri::AppHandle) -> String {
    let dir = dir.to_path_buf();
    let now = Instant::now();
    
    thread::spawn(move || {
        let songs = get_song_files(&dir);
        let songs_metadata = match get_metadata_vec(&songs, app.clone()) {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return format!("Error registering songs: {}", e.to_string());
            }
        };
        app.emit_all("total_songs", crate::Payload { message: songs_metadata.len().to_string() }).unwrap();

        let mut db = Connection::open("D:/Documents/music.db").unwrap();
        let tx = db.transaction().unwrap();

        for song in songs_metadata {
            tx.execute(
                "INSERT OR IGNORE INTO album (cover_path, title, artist, year, genre) 
                VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    &song.cover_path,
                    &song.album_title,
                    &song.album_artist,
                    &song.year,
                    &song.genre,
                ]
            ).unwrap();

            tx.execute(
                "INSERT OR IGNORE INTO song (file_path, cover_path, title, artist, album_title, album_artist, track_number, disc_number, duration, year, genre)
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
            ).unwrap();
        };

        tx.commit().unwrap();

        app.emit_all("songs_registered", crate::Payload { message: "done".to_string() }).unwrap();
        println!("Elapsed time: {:?}", now.elapsed());
        return String::from("Done registering songs");
    });

    return String::from("Registering songs...");
}

#[tauri::command]
pub fn get_all_albums() -> String {
    let db = Connection::open("D:/Documents/music.db");
    println!("{}", "Fetching albums...");
    let now = Instant::now();

    match db {
        Ok(db) => {
            let mut stmt = db.prepare("SELECT title, artist, cover_path FROM album ORDER BY artist, title").unwrap();
            let mut rows = stmt.query(params![]).unwrap();
            
            let mut albums_json = Vec::new();
            while let Some(row) = rows.next().unwrap() {
                let title: String = row.get(0).unwrap();
                let artist: String = row.get(1).unwrap();
                let cover_path: String = row.get(2).unwrap();

                let album = json!({
                    "title": title,
                    "artist": artist,
                    "cover_path": cover_path
                });

                albums_json.push(album);
            }

            println!("Elapsed time: {:?}", now.elapsed());
            return json!(albums_json).to_string();
        },
        Err(_) => return String::from("[]")
    };
}

#[tauri::command]
pub fn get_songs_by_album(title: String, artist: String) -> String {
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let cover_path: String = db.query_row("SELECT cover_path FROM album WHERE title = ?1 AND artist = ?2", params![title, artist], |row| row.get(0)).unwrap();
    let mut stmt = db.prepare("SELECT * FROM song WHERE album_title = ?1 AND artist = ?2 ORDER BY disc_number, track_number").unwrap();
    let mut rows = stmt.query(params![title, artist]).unwrap();
    
    let mut songs_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let file_path: String = row.get(0).unwrap();
        let cover_path: String = row.get(1).unwrap();
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

fn update_metadata(file_path: &str, title: String, artist: String, album: String) -> Result<(), Box<dyn Error>> {
    // let mut tag = Tag::new().read_from_path(&file_path)?;

    // tag.set_title(&title);
    // tag.set_artist(&artist);
    // // tag.set_album(&album);
    // // tag.set_track(track_number);
    // // tag.set_disc(disc_number);
    // tag.write_to_path(file_path)?;

    Ok(())
}

#[tauri::command]
pub fn update_song_info(file_path: String, title: String, artist: String, album: String) -> String {
    let result = update_metadata(&file_path, title, artist, album);
    
    match result {
        Ok(_) => {
            remove_song(file_path);
            return String::from("success");
        },
        Err(e) => {
            println!("{}", e);
            return format!("Error updating metadata: {}", e);
        }
    };
}

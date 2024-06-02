use std::{collections::HashMap, error::Error, path::{Path, PathBuf}};

use audiotags::Tag;
use jwalk::WalkDir;
use rusqlite::{params, Connection};
use serde_json::json;
use tauri::Manager;

use crate::audio;

#[derive(Debug)]
struct AlbumMetadata {
    location_on_disk: String,
    cover_path: Option<String>,
    title: String,
    artist: String,
    year: i32,
    genre: String,
    songs: Vec<SongMetadata>
}

impl Clone for AlbumMetadata {
    fn clone(&self) -> Self {
        AlbumMetadata {
            location_on_disk: self.location_on_disk.clone(),
            cover_path: self.cover_path.clone(),
            title: self.title.clone(),
            artist: self.artist.clone(),
            year: self.year,
            genre: self.genre.clone(),
            songs: self.songs.clone()
        }
    }
}

#[derive(Debug)]
struct SongMetadata {
    parent_dir: String,
    file_path: String,
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
            parent_dir: self.parent_dir.clone(),
            file_path: self.file_path.clone(),
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

fn is_audio_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext {
            "mp3" | "flac" | "m4a" | "ogg" | "wav" => true,
            _ => false
        })
        .unwrap_or_default()
}

fn get_song_count(dir: &Path) -> i32 {
    let mut count = 0;

    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                if is_audio_file(&entry.path()) {
                    count += 1;
                }
            },
            Err(_) => continue
        }
    }

    return count;
}

fn find_cover_art(dir: &Path, album_title: &str) -> Option<String> {
    println!("Searching for cover art in {}", dir.to_string_lossy());
    for entry in WalkDir::new(dir) {
        let Ok(entry) = entry else { continue };
        if !entry.file_type.is_file() {
            continue;
        }

        let lowercase = entry.path().file_stem().unwrap_or_default().to_ascii_lowercase();
        let name = lowercase.to_str().unwrap_or_default();
        let cover_keywords = ["cover", "folder", "front", &album_title.to_ascii_lowercase()];

        if cover_keywords.contains(&name) {
            return Some(entry.path().to_string_lossy().to_string());
        }
    };

    return None;
}

fn get_song_metadata(path: &PathBuf) -> Result<SongMetadata, Box<dyn Error>> {
    let file_path = path.clone().to_string_lossy().to_string();
    let parent_dir = path.parent().ok_or("Failed to get parent directory")?;
    
    let tag = Tag::new().read_from_path(&path)?;
    let title = tag.title().unwrap_or("Unknown").to_owned();
    let artist = tag.artist().unwrap_or("Unknown").to_owned();
    let album_title = tag.album_title().unwrap_or("Unknown").to_owned();
    let album_artist = match tag.album_artist() {
        Some(album_artist) => album_artist.to_owned(),
        None => artist.clone()
    };

    let track_number = tag.track_number().unwrap_or(0);
    let disc_number = tag.disc_number().unwrap_or(0);
    let duration = match tag.duration() {
        // This is really slow, but most songs don't have the duration tag :(
        Some(d) => d as u64,
        None => audio::get_duration(&file_path)
    };

    let year = tag.year().unwrap_or(0);
    let genre = tag.genre().unwrap_or_default().to_string();

    return Ok(SongMetadata {
        parent_dir: parent_dir.to_string_lossy().to_string(),
        file_path,
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

fn commit_to_db(albums: HashMap<String, AlbumMetadata>) -> Result<(), Box<dyn Error>> {
    let mut db = Connection::open("D:/Documents/music.db")?;
    let tx = db.transaction()?;

    for (_, album) in albums {
        tx.execute(
            "INSERT OR REPLACE INTO album (location_on_disk, cover_path, title, artist, year, genre) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &album.location_on_disk,
                &album.cover_path.unwrap_or_default(),
                &album.title,
                &album.artist,
                &album.year,
                &album.genre
            ]
        )?;

        for song in album.songs {
            tx.execute(
                "INSERT OR REPLACE INTO song (file_path, title, artist, album_title, album_artist, track_number, disc_number, duration, year, genre)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    &song.file_path,
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
        }
    };

    tx.commit()?;
    return Ok(());
}

// #[tauri::command]
// pub fn register_file(file_path: &Path) -> Result<String, String> {
//     let file_path = file_path.to_path_buf();
//     let metadata = get_song_metadata(&file_path).map_err(|e| e.to_string())?;
//     let songs_metadata = vec![metadata];

//     commit_to_db(songs_metadata).map_err(|e| e.to_string())?;
//     return Ok("File registered successfully".into());
// }

#[tauri::command]
pub fn register_dir(dir: &Path, app: tauri::AppHandle) -> Result<String, String> {
    let dir = dir.to_path_buf();
    
    let songs = get_song_count(&dir);
    app.emit_all("total_songs", crate::Payload { message: songs.to_string() }).unwrap();

    let mut albums = HashMap::new();
    let mut current_album = AlbumMetadata {
        location_on_disk: String::new(),
        cover_path: None,
        title: String::new(),
        artist: String::new(),
        year: 0,
        genre: String::new(),
        songs: Vec::new()
    };
    let mut successful = 0;
    let mut failed = 0;

    for entry in WalkDir::new(&dir).sort(true) {
        let Ok(entry) = entry else { continue };
        let song_path = entry.path();

        if !is_audio_file(&song_path) { 
            continue;
        }

        match get_song_metadata(&song_path) {
            Ok(metadata) => {
                if current_album.title != metadata.album_title && current_album.artist != metadata.album_artist {
                    if !current_album.songs.is_empty() {
                        albums.insert(current_album.title.clone(), current_album.clone());
                    }

                    let location_on_disk = metadata.parent_dir.clone();
                    let location_path = Path::new(&location_on_disk);
                    let cover_path = find_cover_art(&location_path, &metadata.album_title);

                    current_album = AlbumMetadata {
                        location_on_disk,
                        cover_path,
                        title: metadata.album_title.clone(),
                        artist: metadata.album_artist.clone(),
                        year: metadata.year.clone(),
                        genre: metadata.genre.clone(),
                        songs: Vec::new()
                    };
                }
                current_album.songs.push(metadata);
                successful += 1;
            },
            Err(_) => {
                failed += 1;
                // TODO: keep track of which files failed
            }
        }
        app.emit_all("songs_registered", crate::Payload { message: successful.to_string() }).unwrap();
    };

    commit_to_db(albums).map_err(|e| e.to_string())?;

    let mut message = format!("Registered {} songs", successful);
    if failed > 0 {
        message += format!(", {} could not be read", failed).as_str();
    }
    Ok(message.into())
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
                let location_on_disk: String = row.get(0).unwrap();
                let cover_path: String = row.get(1).unwrap_or_default();
                let title: String = row.get(2).unwrap();
                let artist: String = row.get(3).unwrap();
                let year: u32 = row.get(4).unwrap_or(0);
                let genre: String = row.get(5).unwrap();

                let album = json!({
                    "location_on_disk": location_on_disk,
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
    // TODO: These functions need to be abstracted somehow cause they're almost identical
    let db = Connection::open("D:/Documents/music.db").unwrap();
    let mut stmt = db.prepare("SELECT * FROM album WHERE artist = ?1 ORDER BY title").unwrap();
    let mut rows = stmt.query(params![artist]).unwrap();
    
    let mut albums_json = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let location_on_disk: String = row.get(0).unwrap();
        let cover_path: String = row.get(1).unwrap_or_default();
        let title: String = row.get(2).unwrap();
        let artist: String = row.get(3).unwrap();
        let year: u32 = row.get(4).unwrap_or(0);
        let genre: String = row.get(5).unwrap();

        let album = json!({
            "location_on_disk": location_on_disk,
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

use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use audiotags::Tag;
use jwalk::WalkDir;
use rusqlite::{params, Connection, Params};
use serde_json::Value;
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
    songs: Vec<SongMetadata>,
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
            songs: self.songs.clone(),
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
    genre: String,
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
            genre: self.genre.clone(),
        }
    }
}

fn get_db_connection(app: tauri::AppHandle) -> Result<Connection, Box<dyn Error>> {
    let local_data_dir = app.path().app_data_dir()?;
    let db_path = local_data_dir.join("music.db");

    Ok(Connection::open(db_path)?)
}

fn is_audio_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext {
            "mp3" | "flac" | "m4a" | "ogg" | "wav" => true,
            _ => false,
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
            }
            Err(_) => continue,
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

        let valid_extensions = ["jpg", "png", "jpeg", "gif", "bmp", "webp"];
        if let Some(extension) = entry.path().extension() {
            if !valid_extensions.contains(&extension.to_str().unwrap_or_default()) {
                continue;
            }
        }

        let lowercase = entry
            .path()
            .file_stem()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let name = lowercase.to_str().unwrap_or_default();
        let cover_keywords = [
            "cover",
            "folder",
            "front",
            &album_title.to_ascii_lowercase(),
        ];

        if cover_keywords.contains(&name) {
            return Some(entry.path().to_string_lossy().to_string());
        }
    }

    return None;
}

fn get_song_metadata(path: &PathBuf) -> Result<SongMetadata, Box<dyn Error>> {
    let file_path = path.clone().to_string_lossy().to_string();
    let file_name = path.file_name().ok_or("Failed to get file name")?;
    let file_name = file_name.to_string_lossy().to_string();
    let parent_dir = path.parent().ok_or("Failed to get parent directory")?;

    let tag = Tag::new().read_from_path(&path)?;
    let title = tag.title().unwrap_or(&file_name).to_owned();
    let artist = tag.artist().unwrap_or("Unknown").to_owned();
    let album_title = tag.album_title().unwrap_or("Unknown").to_owned();
    let album_artist = match tag.album_artist() {
        Some(album_artist) => album_artist.to_owned(),
        None => artist.clone(),
    };

    let track_number = match tag.track_number() {
        Some(t) => t,
        None => {
            // Attempt to get track number from first 2 characters in the file path
            let first_two_chars = file_name.clone().chars().take(2).collect::<String>();
            match first_two_chars.parse::<u16>() {
                Ok(t) => t,
                Err(_) => 0,
            }
        },
    };

    let disc_number = tag.disc_number().unwrap_or(0);
    let duration = match tag.duration() {
        Some(d) => {
            // Attempt to convert milliseconds to seconds (PROBABLY NOT RELIABLE)
            // If the duration divided by 1000 is less than 10, assume it's in seconds
            let seconds = (d as u64) / 1000;

            if seconds < 10 {
                d as u64
            } else {
                seconds
            }
        }
        None => audio::get_duration(&file_path)?,
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
        genre,
    });
}

fn commit_to_db(albums: HashMap<String, AlbumMetadata>, app: tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    let mut conn = get_db_connection(app)?;
    let tx = conn.transaction()?;

    for (_, album) in albums {
        let cover_path = album.cover_path.clone().unwrap_or_default();

        tx.execute(
            "INSERT OR REPLACE INTO album (location_on_disk, cover_path, title, artist, year, genre) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &album.location_on_disk,
                &cover_path,
                &album.title,
                &album.artist,
                &album.year,
                &album.genre
            ]
        )?;

        for song in album.songs {
            tx.execute(
                "INSERT OR REPLACE INTO song (file_path, cover_path, title, artist, album_title, album_artist, track_number, disc_number, duration, year, genre)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    &song.file_path,
                    &cover_path,
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
    }

    tx.commit()?;
    return Ok(());
}

#[tauri::command]
pub async fn register_dir(dir: &Path, app: tauri::AppHandle) -> Result<String, String> {
    let dir = dir.to_path_buf();

    let songs = get_song_count(&dir);
    app.emit(
        "total_songs",
        crate::Payload {
            message: songs.to_string(),
        },
    )
    .unwrap();

    let mut albums = HashMap::new();
    let mut current_album = AlbumMetadata {
        location_on_disk: String::new(),
        cover_path: None,
        title: String::new(),
        artist: String::new(),
        year: 0,
        genre: String::new(),
        songs: Vec::new(),
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
                if current_album.title != metadata.album_title || current_album.artist != metadata.album_artist {
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
                        songs: Vec::new(),
                    };

                    albums.insert(current_album.title.clone(), current_album.clone());

                    println!("Found album: {} by {}", current_album.title, current_album.artist);
                }
                albums.get_mut(&current_album.title).unwrap().songs.push(metadata.clone());
                successful += 1;
                app.emit("song_registered", crate::Payload { message: song_path.to_string_lossy().to_string() }).unwrap();
            }
            Err(_) => {
                println!("Failed to read metadata for {}", song_path.to_string_lossy());
                failed += 1;
                // TODO: keep track of which files failed
            }
        }
    }

    commit_to_db(albums, app).map_err(|e| e.to_string())?;

    let mut message = format!("Registered {} songs", successful);
    if failed > 0 {
        message += format!(", {} could not be read", failed).as_str();
    }
    Ok(message.into())
}

fn query_to_json<T: Params>(
    conn: &Connection,
    query: &str,
    params: T,
) -> Result<String, Box<dyn Error>> {
    let mut stmt = conn.prepare(query)?;
    let cached_stmt = conn.prepare_cached(query)?;
    let mut results = Vec::new();
    let mut rows = stmt.query(params)?;

    while let Some(row) = rows.next()? {
        let mut row_map = HashMap::new();
        for (i, col_name) in cached_stmt.column_names().iter().enumerate() {
            let value: Value = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => Value::Null,
                rusqlite::types::ValueRef::Integer(i) => Value::Number(i.into()),
                rusqlite::types::ValueRef::Real(r) => {
                    Value::Number(serde_json::Number::from_f64(r).unwrap())
                }
                rusqlite::types::ValueRef::Text(t) => {
                    Value::String(String::from_utf8(t.to_vec()).unwrap())
                }
                rusqlite::types::ValueRef::Blob(b) => Value::String(hex::encode(b)),
            };
            row_map.insert(col_name.to_string(), value);
        }
        results.push(row_map);
    }

    let json = serde_json::to_string(&results)?;
    Ok(json)
}

fn query_row_params<T: Params>(query: &str, params: T, app: tauri::AppHandle) -> Result<String, String> {
    let conn = get_db_connection(app).map_err(|e| e.to_string())?;
    let json = query_to_json(&conn, query, params).map_err(|e| e.to_string())?;

    Ok(json)
}

fn query_row(query: &str, app: tauri::AppHandle) -> Result<String, String> {
    let conn = get_db_connection(app).map_err(|e| e.to_string())?;
    let json = query_to_json(&conn, query, params![]).map_err(|e| e.to_string())?;

    Ok(json)
}

#[tauri::command]
pub fn get_all_albums(app: tauri::AppHandle) -> Result<String, String> {
    query_row("SELECT * FROM album ORDER BY artist, title", app)
}

#[tauri::command]
pub fn get_albums_by_artist(artist: String, app: tauri::AppHandle) -> Result<String, String> {
    query_row_params(
        "SELECT * FROM album WHERE artist = ?1 ORDER BY title",
        params![artist],
        app
    )
}

#[tauri::command]
pub fn get_all_songs(app: tauri::AppHandle) -> Result<String, String> {
    query_row("SELECT * FROM song ORDER BY album_artist, album_title, disc_number, track_number", app)
}

#[tauri::command]
pub fn get_songs_by_album(title: String, artist: String, app: tauri::AppHandle) -> Result<String, String> {
    query_row_params("SELECT * FROM song WHERE album_title = ?1 AND album_artist = ?2 ORDER BY disc_number, track_number", params![title, artist], app)
}

#[tauri::command]
pub fn get_all_artists(app: tauri::AppHandle) -> Result<String, String> {
    query_row(
        "SELECT 
            artist AS name, 
            COUNT(*) AS album_count,
            (SELECT COUNT(*) FROM song WHERE song.album_artist = album.artist) AS song_count
        FROM album GROUP BY artist ORDER BY artist",
        app
    )
}

#[tauri::command]
pub fn remove_album(album: String, artist: String, app: tauri::AppHandle) -> Result<String, String> {
    let conn = get_db_connection(app).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM song WHERE album_title = ?1 AND album_artist = ?2",
        params![album, artist],
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM album WHERE title = ?1 AND artist = ?2",
        params![album, artist],
    ).map_err(|e| e.to_string())?;

    Ok("Album removed".into())
}

#[tauri::command]
pub fn remove_song(file_path: String, app: tauri::AppHandle) -> Result<String, String> {
    let conn = get_db_connection(app).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM song WHERE file_path = ?1", params![file_path]).map_err(|e| e.to_string())?;
    Ok("Song removed".into())
}

#[tauri::command]
pub fn update_metadata_song(
    location_on_disk: String,
    file_path: String,
    cover_path: String,
    title: String,
    artist: String,
    album_title: String,
    album_artist: String,
    track_number: u16,
    disc_number: u16,
    year: i32,
    genre: String,
    app: tauri::AppHandle
) -> Result<String, String> {
    let mut tag = Tag::new()
        .read_from_path(&file_path)
        .map_err(|e| e.to_string())?;
    let conn = get_db_connection(app).map_err(|e| e.to_string())?;

    tag.set_title(&title);
    tag.set_artist(&artist);
    tag.set_album_title(&album_title);
    tag.set_album_artist(&album_artist);
    tag.set_track_number(track_number);
    tag.set_disc_number(disc_number);
    tag.set_year(year);
    tag.set_genre(&genre);

    tag.write_to_path(&file_path).map_err(|e| e.to_string())?;

    // TOOD: Add option to copy cover art to song directory
    conn.execute(
        "REPLACE INTO album (location_on_disk, cover_path, title, artist)
        VALUES (?1, ?2, ?3, ?4)",
        params![location_on_disk, cover_path, album_title, album_artist],
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE song SET cover_path = ?2, title = ?3, artist = ?4, album_title = ?5, album_artist = ?6, track_number = ?7, disc_number = ?8, year = ?9, genre = ?10
        WHERE file_path = ?1",
        params![file_path, cover_path, title, artist, album_title, album_artist, track_number, disc_number, year, genre]
    ).map_err(|e| e.to_string())?;

    Ok("Song updated".into())
}

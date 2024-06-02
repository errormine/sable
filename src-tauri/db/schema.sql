BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS artist (
    name TEXT UNIQUE PRIMARY KEY NOT NULL,
    album_count INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS album (
    location_on_disk TEXT PRIMARY KEY NOT NULL,
    cover_path TEXT,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    year INTEGER,
    genre TEXT
);

CREATE TABLE IF NOT EXISTS song (
    file_path TEXT PRIMARY KEY NOT NULL,
    cover_path TEXT,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    album_title TEXT NOT NULL,
    album_artist TEXT NOT NULL,
    track_number INTEGER DEFAULT 0,
    disc_number INTEGER DEFAULT 0,
    duration INTEGER DEFAULT 0,
    year INTEGER,
    genre TEXT
);

-- Album count triggers
CREATE TRIGGER IF NOT EXISTS update_album_count_after_insert
AFTER INSERT ON album
BEGIN
    UPDATE artist
    SET album_count = album_count + 1
    WHERE name = NEW.artist;
END;

CREATE TRIGGER IF NOT EXISTS update_album_count_after_delete
AFTER DELETE ON album
BEGIN
    UPDATE artist
    SET album_count = album_count - 1
    WHERE name = OLD.artist;
END;

-- Song cover triggers
CREATE TRIGGER IF NOT EXISTS update_cover_path AFTER UPDATE OF cover_path ON album
FOR EACH ROW
BEGIN
    UPDATE song
    SET cover_path = NEW.cover_path
    WHERE album_title = NEW.title AND album_artist = NEW.artist;
END;

CREATE INDEX IF NOT EXISTS idx_album_artist ON album (artist);
CREATE INDEX IF NOT EXISTS idx_song_album ON song (album_title, album_artist);
COMMIT;
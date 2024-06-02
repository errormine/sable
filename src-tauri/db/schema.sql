BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS album (
    location_on_disk TEXT NOT NULL,
    cover_path TEXT,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    year INTEGER,
    genre TEXT,
    PRIMARY KEY (title, artist)
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

CREATE INDEX IF NOT EXISTS idx_album_artist ON album (artist);
CREATE INDEX IF NOT EXISTS idx_song_album ON song (album_title, album_artist);
COMMIT;
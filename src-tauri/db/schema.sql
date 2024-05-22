BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS album (
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    year INTEGER,
    genre TEXT,
    cover_path TEXT,
    PRIMARY KEY (title, artist)
);

CREATE TABLE IF NOT EXISTS song (
    file_path TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    artist TEXT NOT NULL,
    album TEXT NOT NULL,
    track_number INTEGER DEFAULT 0,
    disc_number INTEGER DEFAULT 0,
    duration INTEGER DEFAULT 0,
    FOREIGN KEY (album, artist) REFERENCES album(title, artist)
);

CREATE INDEX IF NOT EXISTS idx_album_artist ON album (artist);
CREATE INDEX IF NOT EXISTS idx_song_album ON song (album, artist);
COMMIT;
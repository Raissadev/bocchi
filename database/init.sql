CREATE TABLE IF NOT EXISTS plt_video (
    id          INTEGER PRIMARY KEY AUTOINCREMENT
,   id_video    TEXT NOT NULL UNIQUE
,   etag        TEXT NOT NULL
,   kind        TEXT NOT NULL
,   created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
,   posted_at   TIMESTAMP
,   published_at TIMESTAMP
,   channel_id   TEXT
,   title        TEXT
);
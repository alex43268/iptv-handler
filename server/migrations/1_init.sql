CREATE TABLE IF NOT EXISTS Provider
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    description TEXT    NOT NULL,
    m3u         TEXT NOT NULL DEFAULT FALSE
);
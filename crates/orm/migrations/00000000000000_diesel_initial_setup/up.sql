CREATE TABLE IF NOT EXISTS brine (
    key TEXT PRIMARY KEY NOT NULL,
    val TEXT NOT NULL
);

CREATE INDEX brine_key ON brine (key);
CREATE INDEX brine_val ON brine (val);
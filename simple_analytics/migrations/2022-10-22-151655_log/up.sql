-- Your SQL goes here
CREATE TABLE analytics (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  user_agent VARCHAR,
  ip VARCHAR,
  metadata TEXT
);

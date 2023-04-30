CREATE TABLE users (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'localtime')),
  updated_at TIMESTAMP NOT NULL DEFAULT (datetime('now', 'localtime'))
);

CREATE TRIGGER update_user
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
  UPDATE users SET updated_at = datetime('now', 'localtime') WHERE id = OLD.id;
END;

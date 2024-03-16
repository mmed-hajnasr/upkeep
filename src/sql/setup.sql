CREATE TABLE IF NOT EXISTS Machine (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS Component (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 3,
    status INTEGER NOT NULL DEFAULT 1,
    machineid INTEGER NOT NULL,
    FOREIGN KEY (machineid) REFERENCES Machine(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    description TEXT,
    startDate datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status INTEGER NOT NULL DEFAULT 2,
    fixed bit NOT NULL DEFAULT 0,
    fixedDate datetime,
    componentid INTEGER NOT NULL,
    FOREIGN KEY (componentid) REFERENCES Component(id) ON DELETE CASCADE
);

CREATE TRIGGER IF NOT EXISTS updateComponentStatusOnLogUpdate
AFTER
UPDATE ON Log BEGIN
UPDATE Component
SET status = (
        SELECT max(status)
        FROM Log
        WHERE componentid = NEW.componentid
    )
WHERE id = NEW.componentid;
END;

CREATE TRIGGER IF NOT EXISTS updateComponentStatusOnLogInsert
AFTER
INSERT ON Log BEGIN
UPDATE Component
SET status = (
        SELECT max(status)
        FROM Log
        WHERE componentid = NEW.componentid
    )
WHERE id = NEW.componentid;
END;

CREATE TRIGGER IF NOT EXISTS updateComponentStatusOnLogDelete
AFTER
DELETE ON Log BEGIN
UPDATE Component
SET status = (
        SELECT max(status)
        FROM Log
        WHERE componentid = OLD.componentid
    )
WHERE id = OLD.componentid;
END;

CREATE TRIGGER IF NOT EXISTS updateLogNameIfNull
AFTER
INSERT ON Log BEGIN
UPDATE Log
SET name = "Error " || NEW.id
WHERE id = NEW.id AND name IS NULL;
END;

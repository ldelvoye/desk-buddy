CREATE TABLE IF NOT EXISTS hydration_reminder_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message TEXT NOT NULL,
    triggered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS hydration_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    interval_minutes INTEGER NOT NULL
);

INSERT INTO hydration_settings (id, interval_minutes)
VALUES (1, 30)
ON CONFLICT(id) DO NOTHING;

ALTER TABLE hydration_settings
ADD COLUMN snooze_minutes INTEGER NOT NULL DEFAULT 10;

UPDATE hydration_settings
SET snooze_minutes = 10
WHERE snooze_minutes IS NULL OR snooze_minutes < 1;

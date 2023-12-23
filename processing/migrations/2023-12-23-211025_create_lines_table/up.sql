-- Your SQL goes here
-- up.sql
CREATE TABLE lines (
    from_meter_id INTEGER NOT NULL,
    to_meter_id INTEGER NOT NULL,
    feeder_id INTEGER NOT NULL,
    PRIMARY KEY (from_meter_id, to_meter_id),
    FOREIGN KEY (from_meter_id) REFERENCES meter(id),
    FOREIGN KEY (to_meter_id) REFERENCES meter(id),
    FOREIGN KEY (feeder_id) REFERENCES feeder(id)
);

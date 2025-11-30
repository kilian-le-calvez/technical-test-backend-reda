-- Create table for price data
CREATE TABLE IF NOT EXISTS prices
(
    id          SERIAL PRIMARY KEY,
    recorded_at TIMESTAMPTZ    NOT NULL,
    price       NUMERIC(12, 4) NOT NULL
);

-- Index optimized for date-range queries
CREATE INDEX IF NOT EXISTS idx_prices_recorded_at
    ON prices (recorded_at);

-- Insert sample price data across several days
-- Day 1: 2025-01-01
INSERT INTO prices (recorded_at, price)
VALUES ('2025-01-01 09:00:00+00', 101.25),
       ('2025-01-01 12:00:00+00', 102.10),
       ('2025-01-01 15:30:00+00', 99.80);

-- Day 2: 2025-01-02
INSERT INTO prices (recorded_at, price)
VALUES ('2025-01-02 08:45:00+00', 103.50),
       ('2025-01-02 13:20:00+00', 102.75),
       ('2025-01-02 17:00:00+00', 101.10);

-- Day 3: 2025-01-03
INSERT INTO prices (recorded_at, price)
VALUES ('2025-01-03 10:00:00+00', 98.40),
       ('2025-01-03 14:30:00+00', 100.20),
       ('2025-01-03 18:00:00+00', 99.70);

-- Day 4: 2025-01-04
INSERT INTO prices (recorded_at, price)
VALUES ('2025-01-04 09:00:00+00', 104.00),
       ('2025-01-04 11:45:00+00', 104.80),
       ('2025-01-04 16:10:00+00', 105.30);

-- Day 5: 2025-01-05
INSERT INTO prices (recorded_at, price)
VALUES ('2025-01-05 07:50:00+00', 97.20),
       ('2025-01-05 12:00:00+00', 96.90),
       ('2025-01-05 18:30:00+00', 98.10);

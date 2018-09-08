-- Your SQL goes here

CREATE TABLE IF NOT EXISTS exchanges (
       id SERIAL NOT NULL PRIMARY KEY,
       label VARCHAR(32) NOT NULL
);

INSERT INTO exchanges ( label ) VALUES ( 'kraken' );

CREATE TABLE IF NOT EXISTS asset_pairs (
       id SERIAL NOT NULL PRIMARY KEY,
       left_side VARCHAR(5) NOT NULL,
       right_side VARCHAR(5) NOT NULL,
       pair VARCHAR(12) NOT NULL
);

INSERT INTO asset_pairs ( left_side, right_side, pair ) VALUES ( 'BTC', 'USD', 'BTC/USD' );

CREATE TABLE IF NOT EXISTS ticks (
       id BIGSERIAL NOT NULL PRIMARY KEY,
       exchange INTEGER NOT NULL REFERENCES exchanges ( id ),
       asset_pair INTEGER NOT NULL REFERENCES asset_pairs ( id ),
       start_time TIMESTAMP NOT NULL,
       end_time TIMESTAMP NOT NULL,
       first_price NUMERIC(30, 15) NOT NULL,
       first_size NUMERIC(30, 15) NOT NULL,
       highest_price NUMERIC(30, 15) NOT NULL,
       highest_size NUMERIC(30, 15) NOT NULL,
       lowest_price NUMERIC(30, 15) NOT NULL,
       lowest_size NUMERIC(30, 15) NOT NULL,
       last_price NUMERIC(30, 15) NOT NULL,
       last_size NUMERIC(30, 15) NOT NULL,
       trades INTEGER NOT NULL
);

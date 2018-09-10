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

CREATE TABLE IF NOT EXISTS trade_markets (
       id SERIAL NOT NULL PRIMARY KEY,
       market VARCHAR(10) NOT NULL
);

INSERT INTO trade_markets ( market ) VALUES ( 'maker' );
INSERT INTO trade_markets ( market ) VALUES ( 'taker' );

CREATE TABLE IF NOT EXISTS trade_types (
       id SERIAL NOT NULL PRIMARY KEY,
       trade VARCHAR(10) NOT NULL
);

INSERT INTO trade_types ( trade ) VALUES ( 'limit' );
INSERT INTO trade_types ( trade ) VALUES ( 'market' );

CREATE TABLE IF NOT EXISTS trade_history_items (
       id BIGSERIAL NOT NULL PRIMARY KEY,
       exchange INTEGER NOT NULL REFERENCES exchanges ( id ),
       asset_pair INTEGER NOT NULL REFERENCES asset_pairs ( id ),
       happened TIMESTAMP NOT NULL,
       match_size NUMERIC(30,15) NOT NULL,
       match_price NUMERIC(30,15) NOT NULL,
       market INTEGER NOT NULL REFERENCES trade_markets ( id ),
       trade INTEGER NOT NULL REFERENCES trade_types ( id )
)

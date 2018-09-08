table! {
    asset_pairs (id) {
        id -> Int4,
        left_side -> Varchar,
        right_side -> Varchar,
        pair -> Varchar,
    }
}

table! {
    exchanges (id) {
        id -> Int4,
        label -> Varchar,
    }
}

table! {
    ticks (id) {
        id -> Int8,
        exchange -> Int4,
        asset_pair -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        first_price -> Numeric,
        first_size -> Numeric,
        highest_price -> Numeric,
        highest_size -> Numeric,
        lowest_price -> Numeric,
        lowest_size -> Numeric,
        last_price -> Numeric,
        last_size -> Numeric,
        trades -> Int4,
    }
}

joinable!(ticks -> asset_pairs (asset_pair));
joinable!(ticks -> exchanges (exchange));

allow_tables_to_appear_in_same_query!(
    asset_pairs,
    exchanges,
    ticks,
);

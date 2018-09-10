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
    trade_history_items (id) {
        id -> Int8,
        exchange -> Int4,
        asset_pair -> Int4,
        happened -> Timestamp,
        match_size -> Numeric,
        match_price -> Numeric,
        market -> Int4,
        trade -> Int4,
    }
}

table! {
    trade_markets (id) {
        id -> Int4,
        market -> Varchar,
    }
}

table! {
    trade_types (id) {
        id -> Int4,
        trade -> Varchar,
    }
}

joinable!(trade_history_items -> asset_pairs (asset_pair));
joinable!(trade_history_items -> exchanges (exchange));
joinable!(trade_history_items -> trade_markets (market));
joinable!(trade_history_items -> trade_types (trade));

allow_tables_to_appear_in_same_query!(
    asset_pairs,
    exchanges,
    trade_history_items,
    trade_markets,
    trade_types,
);

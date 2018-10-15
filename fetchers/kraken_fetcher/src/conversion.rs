//! Code to convert from internal models to common models.
use std::ops::Mul;

use num_traits::ToPrimitive;
use rust_decimal::Decimal;
use chrono::{NaiveDateTime, DateTime, Utc};

use common::{trade, asset};

use model::{TradeHistory, TradeMatchItem};

lazy_static! {
    static ref NANOS_MUL: Decimal =  1_000_000_000_u64.into();
}

/// Convert the internal kraken trade match history model into the common model. This is
/// done for transmission.
pub fn trade_history(
    history: &TradeHistory
) -> Result<(asset::Pair, Vec<trade::TradeHistoryItem>), String> {
    let mut output: Vec<trade::TradeHistoryItem> = Vec::new();

    // Get the asset pair. All the items are of the same asset pair
    let asset_pair = history.pair();
    
    for trade_match in history.items().iter() {
        // Each trade_match is a vector of six elements.
        // TODO: Consider... could we define a minimum of five elements and any higher than
        //       six are ignored?
        if trade_match.len() != 6 {
            return Err("Insufficient vector length".to_string())
        }

        // 1st is the price
        let price: Decimal = if let TradeMatchItem::Text(ref p) = trade_match[0] {
            p.parse().map_err(|_| "Invalid number format.".to_string())?
        } else {
            return Err("Invalid price item at index 0.".to_string());
        };

        // 2nd is the size
        let size: Decimal = if let TradeMatchItem::Text(ref s) = trade_match[1] {
            s.parse().map_err(|_| "Invalid number format.".to_string())?
        } else {
            return Err("Invalid size item at index 1.".to_string());
        };

        // 3rd is the timestamp
        let ts: DateTime<Utc> = if let TradeMatchItem::Timestamp(ts) = trade_match[2] {
            // First, we need to split the second and millison second components.
            let seconds = ts
                .trunc()
                .to_i64()
                .ok_or("Seconds exceed i64 in timestamp at index 2.".to_owned())?;

            // It's really being stored as milli seconds but chrono only takes nanos when
            // building a timestamp so we need to do a big mul to bring the fractional out.
            let nanos = ts
                .fract()
                .mul(*NANOS_MUL)
                .trunc()
                .to_u32()
                .ok_or("Nansoseconds exceed u32 in timestamp at index 2.".to_owned())?;

            // Now we build out our chrono object            
            DateTime::from_utc(NaiveDateTime::from_timestamp(seconds, nanos), Utc)
        } else {
            return Err("Timestamp must be a number field at index 2.".to_string());
        };

        // 4th is the side
        let side: trade::Market = if let TradeMatchItem::Text(ref t) = trade_match[3] {
            t.parse().map_err(|_| "Invalid market side.".to_string())?
        } else {
            return Err("Market side must be a string at index 3.".to_string());
        };

        // 5th is the type
        let trade: trade::Type = if let TradeMatchItem::Text(ref t) = trade_match[4] {
            t.parse().map_err(|_| "Invalid trade type.".to_string())?
        } else {
            return Err("Trade type must be a string at index 4.".to_string());
        };

        // 6th is the meta (usually blank).
        let _meta = if let TradeMatchItem::Text(ref s) = trade_match[5] {
            s.to_owned()
        } else {
            return Err("Meta data must be a string at index 5.".to_string());
        };

        // Build the TradeHistoryItem and add it to our output. The last four `None`s are
        // for optional data that is not provided by kraken.
        output.push(trade::TradeHistoryItem::new(
            ts, size, price, side, trade, None, None, None, None
        ));
    }

    Ok((asset_pair, output))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    
    use serde_json;    

    use model::Items;
    use super::*;

    #[test]
    fn can_convert_trade_history() {
        let items = vec![
            vec![
                TradeMatchItem::Text("10".to_owned()),
                TradeMatchItem::Text("1".to_owned()),
                TradeMatchItem::Timestamp(Decimal::from_str("123456.1234").unwrap()),
                TradeMatchItem::Text("b".to_owned()),
                TradeMatchItem::Text("m".to_owned()),
                TradeMatchItem::Text(String::new()),
            ],
            vec![
                TradeMatchItem::Text("20".to_owned()),
                TradeMatchItem::Text("2".to_owned()),
                TradeMatchItem::Timestamp(1535271158.into()),
                TradeMatchItem::Text("s".to_owned()),
                TradeMatchItem::Text("l".to_owned()),
                TradeMatchItem::Text(String::new()),
            ],
        ];

        let th = TradeHistory::new(Items::XXBTZUSD(items), "123456".to_owned());
        let result = trade_history(&th);
        
        assert!(result.is_ok());

        let trade_history = result.unwrap();

        assert!(trade_history.len() == 2);
        
        assert!(trade_history[0].size() == 1.into());
        assert!(trade_history[0].price() == 10.into());
        assert!(trade_history[0].market() == trade::Market::Taker);
        assert!(trade_history[0].trade() == trade::Type::Market);

        assert!(trade_history[1].size() == 2.into());
        assert!(trade_history[1].price() == 20.into());
        assert!(trade_history[1].market() == trade::Market::Maker);
        assert!(trade_history[1].trade() == trade::Type::Limit);
    }
}

//! Payloads from binance.

use rust_decimal::Decimal;

/// Universal enum for all Binance websocket payloads. Payloads are internally tagged so
/// representing with an enum is straightforward.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum Payload {
    #[serde(rename = "trade")]
    Trade {
        #[serde(rename = "E")]
        event_time: u64,
        #[serde(rename = "s")]
        symbol: String,
        #[serde(rename = "t")]
        trade_id: u64,
        #[serde(rename = "p")]
        price: Decimal,
        #[serde(rename = "q")]
        quantity: Decimal,
        #[serde(rename = "b")]
        buyer_order_id: u64,
        #[serde(rename = "a")]
        seller_order_id: u64,
        #[serde(rename = "T")]
        trade_time: u64,
        #[serde(rename = "m")]
        market_buyer: bool,
        #[serde(rename = "M")]
        ignore: bool,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamItem {
    stream: String,
    data: Payload,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    #[test]
    fn trade_payload_deserialize() {
        let json1 = r##"{"e":"trade","E":1539264159120,"s":"BNBBTC","t":29661698,"p":"0.00152100","q":"1.00000000","b":84391627,"a":84391631,"T":1539264159104,"m":true,"M":true}"##;

        let payload: Payload = serde_json::from_str(json1).unwrap();
        let back = serde_json::to_string(&payload).unwrap();
        assert_eq!(back.as_str(), json1);
    }

    #[test]
    fn stream_item_deserialize() {
        let json1 = r##"{"stream":"bnbbtc@trade","data":{"e":"trade","E":1539269223771,"s":"BNBBTC","t":29665499,"p":"0.00153050","q":"1.00000000","b":84402623,"a":84402606,"T":1539269223772,"m":false,"M":true}}"##;

        let item: StreamItem = serde_json::from_str(json1).unwrap();
        let back = serde_json::to_string(&item).unwrap();
        assert_eq!(back.as_str(), json1);
    }
}

//! Functions used in handlers.

use chrono::Utc;

//use common::time_util;

use model::TicksRequest;

pub fn prepare_folder_requests(
    folder_url: &str, ticks_request: &TicksRequest,
) -> Vec<String> {
    let from = *ticks_request.from();
    
    let to = ticks_request
        .to()
        .unwrap_or(Utc::now().timestamp() as u64);

    let span = *ticks_request.span() as u64;
    
    // Handle some easy cases first.
    if from >= to {
        // In the future it might be interesting to read ticks backwards, but not now.
        return Vec::new();
    }

    if span == 0 {
        // We'd have a infinite list of empty ticks.
        return Vec::new();
    }

    // Determine periods within the time range
    let delta = to - from;

    // We get the full periods
    let periods = delta / span;

    // We get the remaining partial period if any and increment the periods
    let partial = delta % span;
    let periods = if partial > 0 {
        periods + 1
    } else {
        periods
    };

    // Generate list of request parameters
    let mut req_urls: Vec<String> = Vec::new();
    for period in 0..periods {
        let start_timestamp = from + (span * period);
        let end_timestamp = start_timestamp + span;
        let url = format!(
            "{}/trade_history/{}/{}/{}/tick?from={}&to={}",
            folder_url,
            ticks_request.asset_pair().left(),
            ticks_request.asset_pair().right(),
            ticks_request.exchange(),
            start_timestamp,
            end_timestamp,
        );
        req_urls.push(url);
    }

    req_urls
}

#[cfg(test)]
mod tests {
    use common::{asset, exchange};
    
    use super::*;    

    #[test]
    fn invalid_from_to() {
        let req = TicksRequest::_new(
            exchange::Exchange::Binance,
            asset::BTC_USD,
            10,
            Some(9),
            1
        );

        let params = prepare_folder_requests("y", &req);

        assert!(params.is_empty());
    }

    #[test]
    fn invalid_span() {
        let req = TicksRequest::_new(
            exchange::Exchange::Binance,
            asset::BTC_USD,
            10,
            Some(11),
            0
        );

        let params = prepare_folder_requests("y", &req);

        assert!(params.is_empty());
    }
}

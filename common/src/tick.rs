use rust_decimal::Decimal;

/// A single tick. The time from/to, asset_pair, exchange(s) are not present and to be
/// determined via the calling context.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Tick {
    first: Decimal,
    high: Decimal,
    low: Decimal,
    last: Decimal,
    count: u64,
}

impl Tick {
    pub fn new(
        first: Decimal,
        high: Decimal,
        low: Decimal,
        last: Decimal,
        count: u64
    ) -> Self {
        Tick {
            first, high, low, last, count,
        }
    }

    /// Struct needs to be initialized with the count set to 0 for this method to work
    /// properly as it uses the count to determine the start.
    pub fn folding_add(&mut self, number: Decimal) {
        if self.count > 0 {
            if number > self.high { self.high = number; }
            if number < self.low { self.low = number; }
            self.last = number;
        } else {
            self.first = number;
            self.high = number;
            self.low = number;
            self.last = number;
        }

        self.count += 1;
    }
}

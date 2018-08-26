//! Asset types.

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Asset {
    BTC,
    ETH,
    USD,
    BNB,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pair {
    left: Asset,
    right: Asset,
}

impl Pair {
    pub fn new(left: Asset, right: Asset) -> Self {
        Pair { left, right }
    }

    pub fn left(&self) -> Asset {
        self.left
    }

    pub fn right(&self) -> Asset {
        self.right
    }
}

macro_rules! asset_pair {
    ($name:ident, $left:expr, $right:expr) => {
        pub const $name: Pair = Pair { left: $left, right: $right };
    };
}

asset_pair!(BTC_USD, Asset::BTC, Asset::USD);

use crate::TradeId;
use derive_more::Into;
use rust_decimal::Decimal;
use timestamp_please::{MILLI, Timestamp};

#[derive(Into, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Trade {
    pub id: TradeId,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: Timestamp<u64, MILLI>,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

impl Trade {}

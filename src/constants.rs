pub const TICKERS: [&str; 6] = ["TSLA", "AAPL", "MSFT", "AMD", "INTC", "NVDA"];
pub const TICKER: &str = "NVDA";

pub mod rsi {
    pub const MIN_SELL: f64 = 60.;
    pub const MAX_BUY: f64 = 40.;
    pub const MOVING_AVG_DAYS: u32 = 14;
}

pub mod files {
    pub const DATA_PATH: &str = "data/";
    pub const WEIGHTS_PATH: &str = "weights/";
}

pub mod agent {
    pub const LEARNING_RATE: f64 = 0.005;
    /// How many agents we want in training at each training step
    pub const TARGET_AGENT_COUNT: u32 = 100;
    /// How many generations to run to train the agents
    pub const TARGET_GENERATIONS: u32 = 100;
}

/// The maximum amount of assets change that can be made in a single position in a trade
pub const MAX_CHANGE: f64 = 1.;
/// A ticker may have no more than this percept in total assets
pub const MAX_VALUE_PER_TICKER: f64 = 0.1;
/// Preference for percent of which to buy
pub const BUY_WEIGHT: f64 = 0.8;
/// Preference for percent of which to sell
pub const SELL_WEIGHT: f64 = 0.3;
//! Exc: Abstractions for exchanges.
#![deny(missing_docs)]

/// Instruments Layer.
#[cfg(feature = "instrument")]
pub mod instrument;

/// Types.
pub mod types;

/// Utils for using low-level apis ([`exc::core`](crate::core)).
pub mod util;

pub use self::core::{
    service::adapt::AdaptLayer, Adaptor, Exc, ExcLayer, ExcService, ExchangeError, IntoExc, Request,
};
pub use exc_core as core;
pub use util::{
    book::SubscribeBidAskService,
    fetch_candles::FetchCandlesService,
    instrument::{FetchInstrumentsService, SubscribeInstrumentsService},
    subscribe_tickers::SubscribeTickersService,
    trade::SubscribeTradesService,
    trading::{CheckOrderService, SubscribeOrdersService, TradingService},
    ExcExt,
};

/// Prelude.
pub mod prelude {
    pub use crate::core::{
        types::{Period, Place, PlaceOrderOptions},
        Adaptor, Exc, ExcService, ExcServiceExt, ExchangeError, Request,
    };
    pub use crate::util::{
        book::SubscribeBidAskService,
        fetch_candles::{FetchCandlesService, FetchCandlesServiceExt},
        instrument::{FetchInstrumentsService, SubscribeInstrumentsService},
        reconnect::ReconnectService,
        subscribe_tickers::SubscribeTickersService,
        trade::SubscribeTradesService,
        trading::{CheckOrderService, SubscribeOrdersService, TradingService},
        ExcExt,
    };
}

/// The result type of `exc`.
pub type Result<T> = std::result::Result<T, ExchangeError>;

#[cfg(feature = "retry")]
pub use crate::core::retry;

//! Exc: Abstractions for exchanges.
#![deny(missing_docs)]

/// Instruments Layer.
#[cfg(feature = "instrument")]
pub mod instrument;

/// Types.
pub mod types;

pub use self::core::{
    service::adapt::AdaptLayer, Adaptor, Exc, ExcLayer, ExcService, ExchangeError, IntoExc, Request,
};
pub use exc_core as core;

/// Prelude.
pub mod prelude {
    pub use crate::core::{
        types::{Period, Place, PlaceOrderOptions},
        Adaptor, Exc, ExcService, ExcServiceExt, ExchangeError, Request,
    };
}

/// The result type of `exc`.
pub type Result<T> = std::result::Result<T, ExchangeError>;

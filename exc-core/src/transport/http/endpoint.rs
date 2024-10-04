/// Endpoint.
#[derive(Debug, Default)]
pub struct Endpoint {}

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
mod https {
    use super::*;
    use crate::transport::http::channel::HttpsChannel;
    use reqwest::Client;

    impl Endpoint {
        /// Create a https channel.
        pub fn connect_https(&self) -> HttpsChannel {
            let client = Client::builder().redirect(reqwest::redirect::Policy::none());
            HttpsChannel {
                inner: client.build().unwrap(),
            }
        }
    }
}

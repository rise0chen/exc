/// Endpoint.
#[derive(Debug, Default)]
pub struct Endpoint {}

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
mod https {
    use super::*;
    use crate::transport::http::channel::HttpsChannel;
    use hyper_util::client::legacy::Client;
    use hyper_util::rt::TokioExecutor;

    impl Endpoint {
        /// Create a https channel.
        pub fn connect_https(&self) -> HttpsChannel {
            cfg_if::cfg_if! {
                if #[cfg(feature = "native-tls")] {
                    let https = hyper_tls::HttpsConnector::new();
                } else if #[cfg(feature = "rustls-tls")] {
                    let https = hyper_rustls::HttpsConnectorBuilder::new().with_webpki_roots().https_or_http();
                    #[cfg(not(feature = "http2"))]
                    let https = https.enable_http1();
                    #[cfg(feature = "http2")]
                    let https = https.enable_all_versions();
                    let https= https.build();
                }
            }

            let mut client = Client::builder(TokioExecutor::new());
            client.retry_canceled_requests(false);
            #[cfg(feature = "http2")]
            client.http2_only(true);
            let client = client.build(https);
            HttpsChannel { inner: client }
        }
    }
}

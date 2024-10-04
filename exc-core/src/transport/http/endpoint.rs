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
                    use hyper_rustls::ConfigBuilderExt;

                    let https = hyper_rustls::HttpsConnectorBuilder::new();
                    let https = https.with_tls_config(rustls::ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13]).with_webpki_roots().with_no_client_auth());
                    let https = https.https_or_http();
                    #[cfg(not(feature = "http2"))]
                    let https = https.enable_http1();
                    #[cfg(feature = "http2")]
                    let https = https.enable_http2();
                    let https= https.build();
                }
            }

            let mut client = Client::builder(TokioExecutor::new());
            client.retry_canceled_requests(false);
            let client = client.build(https);
            HttpsChannel { inner: client }
        }
    }
}

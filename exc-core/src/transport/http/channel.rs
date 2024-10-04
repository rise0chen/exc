#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
pub use https::HttpsChannel;

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
/// Https channel.
pub mod https {
    use crate::ExchangeError;
    use futures::{future::BoxFuture, FutureExt, TryFutureExt};
    use reqwest::{Client,Request, Response};

    /// Https channel.
    #[derive(Clone)]
    pub struct HttpsChannel {
        pub(crate) inner: Client,
    }

    impl tower::Service<Request> for HttpsChannel {
        type Response = Response;
        type Error = ExchangeError;
        type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

        fn poll_ready(
            &mut self,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx).map_err(ExchangeError::Http)
        }

        fn call(&mut self, req: Request) -> Self::Future {
            tower::Service::call(&mut self.inner, req)
                .map_err(ExchangeError::Http)
                .boxed()
        }
    }
}

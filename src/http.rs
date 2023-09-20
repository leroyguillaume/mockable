use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{header::HeaderMap, Client, Method, Response, Result, StatusCode};
use tracing::trace;

// HttpRequest

/// A HTTP request.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpRequest {
    /// The headers to send with the request.
    pub headers: HeaderMap,
    /// The HTTP method to use.
    pub method: Method,
    /// The query parameters to send with the request.
    pub query: HashMap<String, String>,
    /// The URL to send the request to.
    pub url: String,
}

// HttpClient

/// A trait for sending HTTP requests.
///
/// **This is supported on `feature=http-client` only.**
///
/// # Examples
/// ```
/// use mockall::predicate::eq;
/// use mockable::{DefaultHttpClient, HttpClient, HttpRequest, HttpResponse, MockHttpClient, MockHttpResponse};
/// use reqwest::{Method, Result, StatusCode};
///
/// async fn send(req: HttpRequest, client: &dyn HttpClient) -> Result<Box<dyn HttpResponse>> {
///     client.send(req).await
/// }
///
/// tokio_test::block_on(async {
///     let req = HttpRequest {
///         headers: Default::default(),
///         method: Method::GET,
///         query: Default::default(),
///         url: "https://google.com".to_string(),
///     };
///
///     // Default
///     let client = DefaultHttpClient;
///     let resp = send(req.clone(), &client).await.unwrap();
///     assert!(resp.status().is_success());
///
///     // Mock
///     let mut client = MockHttpClient::new();
///     client
///         .expect_send()
///         .with(eq(req.clone()))
///         .returning(|_| {
///             let mut resp = MockHttpResponse::new();
///             resp
///                 .expect_status()
///                 .returning(|| StatusCode::OK);
///             Ok(Box::new(resp))
///         });
///     let resp = send(req, &client).await.unwrap();
///     assert!(resp.status().is_success());
/// });
/// ```
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// Send a HTTP request.
    async fn send(&self, req: HttpRequest) -> Result<Box<dyn HttpResponse>>;
}

// HttpResponse

/// A HTTP response.
///
/// **This is supported on `feature=http-client` only.**
#[async_trait]
pub trait HttpResponse {
    /// Get the response body.
    async fn body(self: Box<Self>) -> Result<Bytes>;

    /// Get the response headers.
    fn headers(&self) -> &HeaderMap;

    /// Convert the response into a [`reqwest::Response`](https://docs.rs/reqwest/latest/reqwest/struct.Response.html).
    fn into_response(self: Box<Self>) -> Response;

    /// Get the response status code.
    fn status(&self) -> StatusCode;
}

// DefaultHttpClient

/// Default implementation of [`HttpClient`](trait.HttpClient.html).
///
/// **This is supported on `feature=http-client` only.**
pub struct DefaultHttpClient;

#[async_trait]
impl HttpClient for DefaultHttpClient {
    async fn send(&self, req: HttpRequest) -> Result<Box<dyn HttpResponse>> {
        trace!(?req, "sending HTTP request");
        let resp = Client::new()
            .request(req.method, &req.url)
            .headers(req.headers)
            .query(&req.query)
            .send()
            .await?;
        Ok(Box::new(DefaultHttpResponse(resp)))
    }
}

// DefaultHttpResponse

/// Default implementation of [`HttpResponse`](trait.HttpResponse.html).
///
/// **This is supported on `feature=http-client` only.**
pub struct DefaultHttpResponse(Response);

impl From<Response> for DefaultHttpResponse {
    fn from(resp: Response) -> Self {
        Self(resp)
    }
}

#[async_trait]
impl HttpResponse for DefaultHttpResponse {
    async fn body(self: Box<Self>) -> Result<Bytes> {
        trace!("reading HTTP response body");
        self.0.bytes().await
    }

    fn headers(&self) -> &HeaderMap {
        self.0.headers()
    }

    fn into_response(self: Box<Self>) -> Response {
        self.0
    }

    fn status(&self) -> StatusCode {
        self.0.status()
    }
}

// MockHttpClient

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`HttpClient`](trait.HttpClient.html).
    ///
    /// **This is supported on `feature=http-client,mock` only.**
    pub HttpClient {}

    #[async_trait]
    impl HttpClient for HttpClient {
        async fn send(&self, req: HttpRequest) -> Result<Box<dyn HttpResponse>>;
    }
}

// MockHttpResponse

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`HttpResponse`](trait.HttpResponse.html).
    ///
    /// **This is supported on `feature=http-client,mock` only.**
    pub HttpResponse {}

    #[async_trait]
    impl HttpResponse for HttpResponse {
        async fn body(self: Box<Self>) -> Result<Bytes>;

        fn headers(&self) -> &HeaderMap;

        fn into_response(self: Box<Self>) -> Response;

        fn status(&self) -> StatusCode;
    }
}

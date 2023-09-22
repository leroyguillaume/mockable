use std::collections::HashMap;

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Method, Response, Result, StatusCode,
};
use tracing::trace;

// HttpRequest

/// A HTTP request.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HttpRequest {
    /// The request body.
    pub body: Vec<u8>,
    /// The headers to send with the request.
    pub headers: HeaderMap,
    /// The HTTP method to use.
    pub method: Method,
    /// The query parameters to send with the request.
    pub query: HashMap<String, String>,
    /// The URL to send the request to.
    pub url: String,
}

impl HttpRequest {
    /// Create a new DELETE HTTP request.
    pub fn delete(url: String) -> Self {
        Self::new(Method::DELETE, url)
    }

    /// Create a new GET HTTP request.
    pub fn get(url: String) -> Self {
        Self::new(Method::GET, url)
    }

    /// Create a new PATCH HTTP request.
    pub fn patch(url: String) -> Self {
        Self::new(Method::PATCH, url)
    }

    /// Create a new POST HTTP request.
    pub fn post(url: String) -> Self {
        Self::new(Method::POST, url)
    }

    /// Create a new PUT HTTP request.
    pub fn put(url: String) -> Self {
        Self::new(Method::PUT, url)
    }

    /// Create a new HTTP request.
    pub fn new(method: Method, url: String) -> Self {
        Self {
            body: vec![],
            headers: Default::default(),
            method,
            query: Default::default(),
            url,
        }
    }

    /// Set body.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    /// Set header.
    pub fn with_header(mut self, name: HeaderName, val: HeaderValue) -> Self {
        self.headers.insert(name, val);
        self
    }

    /// Set all headers.
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    /// Set query parameter.
    pub fn with_param(mut self, name: String, value: String) -> Self {
        self.query.insert(name, value);
        self
    }

    /// Set all query parameters.
    pub fn with_query(mut self, query: HashMap<String, String>) -> Self {
        self.query = query;
        self
    }
}

// HttpClient

/// A trait for sending HTTP requests.
///
/// **This is supported on `feature=http-client` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/http.rs).
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
pub trait HttpResponse: Send + Sync {
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
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/http.rs).
pub struct DefaultHttpClient;

#[async_trait]
impl HttpClient for DefaultHttpClient {
    async fn send(&self, req: HttpRequest) -> Result<Box<dyn HttpResponse>> {
        trace!(?req, "sending HTTP request");
        let resp = Client::new()
            .request(req.method, &req.url)
            .headers(req.headers)
            .query(&req.query)
            .body(req.body)
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
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/http.rs).
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

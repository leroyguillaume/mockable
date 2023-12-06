use std::{collections::HashMap, io, net::SocketAddr};

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::Query,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{Html, IntoResponse},
    Json, Router, Server,
};
use serde_json::Value;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::{error, warn};

// HttpRequest

/// HTTP request.
///
/// **This is supported on `feature=http` only.**
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest {
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub method: String,
    pub path: String,
    pub query: HashMap<String, Vec<String>>,
}

// HttpResponse

/// HTTP response.
///
/// **This is supported on `feature=http` only.**
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HttpResponse {
    Empty,
    Html(String),
    Json(Value),
    Text(String),
}

// HttpServer

/// Simple HTTP server that listen all requests.
///
/// **This is supported on `feature=http` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/http.rs).
#[async_trait]
pub trait HttpServer: Send + Sync {
    /// Returns the next request received by the server.
    ///
    /// `None` is returned if the server is stopped.
    async fn next(&mut self) -> Option<HttpRequest>;

    /// Stops the server.
    async fn stop(self);
}

// DefaultHttpServer

/// Default implementation of [`HttpServer`](trait.HttpServer.html).
///
/// **This is supported on `feature=http` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/http.rs).
pub struct DefaultHttpServer {
    req_rx: mpsc::Receiver<HttpRequest>,
    server: JoinHandle<()>,
    stop_tx: oneshot::Sender<()>,
}

impl DefaultHttpServer {
    /// Starts a new server listening on the given address.
    ///
    /// The server will respond status code 200 with an empty response to all requests.
    pub async fn start(addr: &SocketAddr) -> io::Result<Self> {
        Self::with_response(addr, HttpResponse::Empty).await
    }

    /// Starts a new server listening on the given address.
    ///
    /// The server will respond status code 200 with the given one to all requests.
    pub async fn with_response(addr: &SocketAddr, resp: HttpResponse) -> io::Result<Self> {
        let (stop_tx, stop_rx) = oneshot::channel();
        let (req_tx, req_rx) = mpsc::channel(1);
        let app = Router::new().fallback(
            move |method: Method,
                  uri: Uri,
                  Query(query): Query<Vec<(String, String)>>,
                  headers: HeaderMap,
                  body: Bytes| async move {
                let mut req_headers = HashMap::new();
                for (name, val) in headers {
                    let name = if let Some(name) = &name {
                        name.as_str()
                    } else {
                        warn!("request contains header with no name");
                        continue;
                    };
                    let val = match val.to_str() {
                        Ok(val) => val,
                        Err(err) => {
                            warn!(details = %err, header = name, "failed to decode header value");
                            continue;
                        }
                    };
                    req_headers.insert(name.into(), val.into());
                }
                let query = query.into_iter().fold(
                    HashMap::<String, Vec<String>>::new(),
                    |mut query, (key, val)| {
                        query.entry(key).or_default().push(val);
                        query
                    },
                );
                let req = HttpRequest {
                    body: body.to_vec(),
                    headers: req_headers,
                    method: method.to_string(),
                    path: uri.path().into(),
                    query,
                };
                req_tx.send(req).await.ok();
                match resp {
                    HttpResponse::Empty => StatusCode::OK.into_response(),
                    HttpResponse::Html(html) => (StatusCode::OK, Html(html)).into_response(),
                    HttpResponse::Json(json) => (StatusCode::OK, Json(json)).into_response(),
                    HttpResponse::Text(text) => (StatusCode::OK, text).into_response(),
                }
            },
        );
        let server = Server::bind(addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async {
                stop_rx.await.ok();
            });
        let server = spawn(async {
            if let Err(err) = server.await {
                error!(details = %err, "failed to start server");
            }
        });
        Ok(Self {
            req_rx,
            server,
            stop_tx,
        })
    }
}

#[async_trait]
impl HttpServer for DefaultHttpServer {
    async fn next(&mut self) -> Option<HttpRequest> {
        self.req_rx.recv().await
    }

    async fn stop(self) {
        self.stop_tx.send(()).ok();
        if let Err(err) = self.server.await {
            error!(details = %err, "failed to stop server");
        }
    }
}

// MockHttpServer

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`HttpServer`](trait.HttpServer.html).
    ///
    /// **This is supported on `feature=http,mock` only.**
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/http.rs).
    pub HttpServer {}

    #[async_trait]
    impl HttpServer for HttpServer {
        async fn next(&mut self) -> Option<HttpRequest>;
        async fn stop(self);
    }
}

// Tests

#[cfg(test)]
mod test {
    use std::{
        net::{Ipv4Addr, SocketAddrV4},
        time::Duration,
    };

    use reqwest::{Client, Response};
    use tokio::time::sleep;

    use super::*;

    // Mods

    mod default_http_server {
        use super::*;

        // run

        async fn run(port: u16, resp: HttpResponse) -> Response {
            let expected = HttpRequest {
                body: "abc".to_string().into_bytes(),
                headers: HashMap::from_iter([
                    ("accept".into(), "*/*".into()),
                    ("content-length".into(), "3".into()),
                    ("host".into(), format!("localhost:{port}")),
                ]),
                method: "GET".into(),
                path: "/a/b".into(),
                query: HashMap::from_iter([("foo".into(), vec!["bar1".into(), "bar2".into()])]),
            };
            let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port));
            let mut server = DefaultHttpServer::with_response(&addr, resp)
                .await
                .expect("failed to start server");
            sleep(Duration::from_secs(1)).await;
            let client = Client::new();
            let query: Vec<(String, String)> = expected
                .query
                .clone()
                .into_iter()
                .flat_map(|(key, val)| val.into_iter().map(move |val| (key.clone(), val)))
                .collect();
            let resp = client
                .get(format!("http://localhost:{port}{}", expected.path))
                .query(&query)
                .body(expected.body.clone())
                .send()
                .await
                .expect("failed to send request");
            let status = resp.status();
            if status != reqwest::StatusCode::OK {
                let body = resp.text().await.expect("failed to read response body");
                panic!("request failed with status {status}: {body}");
            }
            let req = server.next().await.expect("failed to receive request");
            assert_eq!(req, expected);
            server.stop().await;
            client
                .get(format!("http://localhost:{port}"))
                .send()
                .await
                .expect_err("request should fail after server is stopped");
            resp
        }

        // Tests

        #[tokio::test]
        async fn empty() {
            let resp = run(8000, HttpResponse::Empty).await;
            let text = resp.text().await.expect("failed to read response body");
            assert!(text.is_empty());
        }

        #[tokio::test]
        async fn html() {
            let expected = "<head></head>";
            let resp = run(8001, HttpResponse::Html(expected.into())).await;
            let text = resp.text().await.expect("failed to read response body");
            assert_eq!(text, expected);
        }

        #[tokio::test]
        async fn json() {
            let expected = Value::String("val".into());
            let resp = run(8002, HttpResponse::Json(expected.clone())).await;
            let json: Value = resp.json().await.expect("failed to read response body");
            assert_eq!(json, expected);
        }

        #[tokio::test]
        async fn text() {
            let expected = "val";
            let resp = run(8003, HttpResponse::Text(expected.into())).await;
            let text = resp.text().await.expect("failed to read response body");
            assert_eq!(text, expected);
        }
    }
}

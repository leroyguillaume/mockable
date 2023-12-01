use std::{collections::HashMap, io, net::SocketAddr};

use async_trait::async_trait;
use axum::{
    body::Bytes,
    extract::Query,
    http::{HeaderMap, Method, StatusCode, Uri},
    Router, Server,
};
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tracing::{error, warn};

// HttpRequest

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest {
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub method: String,
    pub path: String,
    pub query: HashMap<String, Vec<String>>,
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
    pub async fn start(addr: &SocketAddr) -> io::Result<Self> {
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
                StatusCode::OK
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

    use reqwest::Client;
    use tokio::time::sleep;

    use super::*;

    mod default_http_server {
        use super::*;

        #[tokio::test]
        async fn test() {
            let port = 8000;
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
            let mut server = DefaultHttpServer::start(&addr)
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
        }
    }
}

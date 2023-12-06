use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use mockable::{DefaultHttpServer, HttpResponse, HttpServer};

#[tokio::main]
async fn main() {
    let html = "<html><body><h1>Hello, world!</h1></body></html>";
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8000));
    let mut server = DefaultHttpServer::with_response(&addr, HttpResponse::Html(html.into()))
        .await
        .expect("failed to start server");
    let req = server.next().await.expect("failed to get request");
    println!("{:?}", req);
    server.stop().await;
}

#[cfg(test)]
mod test {
    use mockable::{HttpRequest, MockHttpServer};

    use super::*;

    #[tokio::test]
    async fn test() {
        let expected = HttpRequest {
            body: vec![],
            headers: Default::default(),
            method: "GET".into(),
            path: "/".into(),
            query: Default::default(),
        };
        let mut server = MockHttpServer::new();
        server.expect_next().return_const(expected.clone());
        server.expect_stop().return_const(());
        let req = server.next().await.expect("failed to get request");
        server.stop().await;
        assert_eq!(req, expected);
    }
}

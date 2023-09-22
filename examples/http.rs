use mockable::{DefaultHttpClient, HttpClient, HttpRequest};
use reqwest::Method;

struct GoogleClient(Box<dyn HttpClient>);

impl GoogleClient {
    fn new() -> Self {
        Self(Box::new(DefaultHttpClient))
    }

    async fn get(&self) -> String {
        let req = HttpRequest {
            headers: Default::default(),
            method: Method::GET,
            query: Default::default(),
            url: "https://google.com".into(),
        };
        let resp = self.0.send(req).await.expect("request failed");
        let body = resp.body().await.expect("reading body failed");
        String::from_utf8(body.to_vec()).expect("body is not utf8")
    }
}

#[tokio::main]
async fn main() {
    let client = GoogleClient::new();
    let html = client.get().await;
    println!("{html}")
}

#[cfg(test)]
mod test {
    use mockable::{MockHttpClient, MockHttpResponse};
    use mockall::predicate::eq;

    use super::*;

    #[tokio::test]
    async fn test() {
        let expected = "<html></html>";
        let req = HttpRequest {
            headers: Default::default(),
            method: Method::GET,
            query: Default::default(),
            url: "https://google.com".into(),
        };
        let mut client = MockHttpClient::new();
        client.expect_send().with(eq(req)).returning(|_| {
            let mut resp = MockHttpResponse::new();
            resp.expect_body()
                .returning(|| Ok(expected.as_bytes().into()));
            Ok(Box::new(resp))
        });
        let client = GoogleClient(Box::new(client));
        let html = client.get().await;
        assert_eq!(html, expected);
    }
}

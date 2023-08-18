use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
struct ShowHeaders;

const NON_ASCII_IN_HEADER: &str = "Non-ASCII character(s) in the header.";

impl tower::Service<hyper::Request<hyper::Body>> for ShowHeaders {
    type Response = hyper::Response<hyper::Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + Sync>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
        let mut lines = Vec::with_capacity(req.headers().len());
        for (key, value) in req.headers() {
            let value = match value.to_str() {
                Ok(s) => s,
                Err(_) => NON_ASCII_IN_HEADER,
            };
            lines.push(format!("{}: {}", key.to_string(), value));
        }
        let body = hyper::Body::from(lines.join("\n"));
        let resp = hyper::Response::builder()
            .status(200)
            .header("Content-Type", "text/plain")
            .body(body)
            .expect("Unable to create the `hyper::Response` object");

        let fut = async { Ok(resp) };

        Box::pin(fut)
    }
}

#[shuttle_runtime::main] // previous incorrect: shuttle_service::main
async fn tower() -> shuttle_tower::ShuttleTower<ShowHeaders> {
    // previous incorrect: -> Result<HelloWorld, shuttle_service::Error
    Ok(ShowHeaders.into()) // previous incorrect: Ok(HelloWorld)
}

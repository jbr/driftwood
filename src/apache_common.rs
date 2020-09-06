#![allow(clippy::print_literal)]
use crate::LogMiddlewareHasBeenRun;
use chrono::offset::Local;
use tide::{Request, Result};

/// Apache common log format
///
/// Example:
/// `127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326`
#[derive(Debug, Copy, Clone)]
pub struct ApacheCommonLogger;
#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> tide::Middleware<T> for ApacheCommonLogger {
    async fn handle(&self, mut req: Request<T>, next: tide::Next<'_, T>) -> Result {
        if req.ext::<LogMiddlewareHasBeenRun>().is_some() {
            return Ok(next.run(req).await);
        }
        req.set_ext(LogMiddlewareHasBeenRun);

        let peer_addr = req.peer_addr().map(String::from);
        let path = req.url().path().to_owned();
        let method = req.method().to_string();
        let hr: &tide::http::Request = req.as_ref();
        let protocol = hr.version();
        let start = Local::now();
        let response = next.run(req).await;
        let status = response.status();
        println!(
            r#"{ip} {id} {userid} [{timestamp}] "{method} {path} {version}" {status} {len}"#,
            ip = peer_addr.as_deref().unwrap_or("-"),
            id = "-",
            userid = "-",
            timestamp = start.format("%d/%b/%Y:%H:%M:%S %z"),
            method = method,
            path = path,
            version = match protocol {
                Some(tide::http::Version::Http1_1) => "HTTP/1.1",
                Some(_) => "HTTP",
                None => "-",
            },
            status = status,
            len = response
                .len()
                .map(|l| l.to_string())
                .unwrap_or_else(|| String::from("-")),
        );
        Ok(response)
    }
}

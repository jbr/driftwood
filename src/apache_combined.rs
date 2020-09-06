use crate::LogMiddlewareHasBeenRun;
use chrono::offset::Local;
use tide::{
    http::headers::{REFERER, USER_AGENT},
    Request, Result,
};
/// Apache combined format
/// Example:
/// `127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08 [en] (Win98; I ;Nav)"`
#[derive(Debug, Copy, Clone)]
pub struct ApacheCombinedLogger;

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> tide::Middleware<T> for ApacheCombinedLogger {
    async fn handle(&self, mut req: Request<T>, next: tide::Next<'_, T>) -> Result {
        if req.ext::<LogMiddlewareHasBeenRun>().is_some() {
            return Ok(next.run(req).await);
        }
        req.set_ext(LogMiddlewareHasBeenRun);
        let referrer = req.header(REFERER).map(|r| r.as_str().to_owned());
        let ua = req.header(USER_AGENT).map(|r| r.as_str().to_owned());
        let peer_addr = req.peer_addr().map(String::from);
        let path = req.url().path().to_owned();
        let method = req.method().to_string();
        let hr: &tide::http::Request = req.as_ref();
        let protocol = hr.version();
        let start = Local::now();
        let response = next.run(req).await;
        let status = response.status();
        println!(
            r#"{ip} {id} {userid} [{timestamp}] "{method} {path} {version}" {status} {len} {referrer:?} {ua:?}"#,
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
            referrer = referrer.as_deref().unwrap_or(""),
            ua = ua.as_deref().unwrap_or(""),
        );
        Ok(response)
    }
}

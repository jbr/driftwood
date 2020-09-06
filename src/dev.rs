use crate::LogMiddlewareHasBeenRun;
use colored::*;
use size::{Base, Size, Style};
use tide::{Request, Result};
/// Development logger
///
/// This logger colors the status code based on the status range
/// Example:
/// GET http://localhost:8080/some/path 200 3.289292ms 227bytes

#[derive(Debug, Copy, Clone)]
pub struct DevLogger;
#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> tide::Middleware<T> for DevLogger {
    async fn handle(&self, mut req: Request<T>, next: tide::Next<'_, T>) -> Result {
        if req.ext::<LogMiddlewareHasBeenRun>().is_some() {
            return Ok(next.run(req).await);
        }
        req.set_ext(LogMiddlewareHasBeenRun);

        let url = req.url().to_owned();
        let method = req.method().to_string();
        let start = std::time::Instant::now();
        let response = next.run(req).await;
        let status = response.status();
        println!(
            r#"{method} {url} {status} {response_time:?} {len}"#,
            response_time = std::time::Instant::now() - start,
            method = method,
            url = url,
            status = status.to_string().color(match status.into() {
                200..=299 => "green",
                300..=399 => "cyan",
                400..=499 => "yellow",
                500..=599 => "red",
                _ => "white",
            }),
            len = response
                .len()
                .map(
                    |l| Size::to_string(&Size::Bytes(l), Base::Base10, Style::Smart)
                        .replace(" ", "")
                )
                .unwrap_or_else(|| String::from("-")),
        );
        Ok(response)
    }
}

#![allow(clippy::print_literal)]
use chrono::offset::Local;
use colored::*;
use tide::{
    http::headers::{REFERER, USER_AGENT},
    Request, Result,
};

use size::{Base, Size, Style};

struct LogMiddlewareHasBeenRun;

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

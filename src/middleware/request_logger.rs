// use actix_web::{self, middleware::Started, HttpMessage, HttpRequest};
use crate::share::state::AppState;

use chrono::{NaiveDateTime, Utc};
// use time;

use actix_web::{
    error::Result,
    middleware::{Finished, Middleware, Started},
    HttpRequest, HttpResponse,
};

pub struct RequestLogger;

struct StartTime(NaiveDateTime);

impl Middleware<AppState> for RequestLogger {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        req.extensions_mut()
            .insert(StartTime(Utc::now().naive_utc()));
        Ok(Started::Done)
    }

    fn finish(&self, req: &HttpRequest<AppState>, resp: &HttpResponse) -> Finished {
        if let Some(entry_time) = req.extensions().get::<StartTime>() {
            let elapsed_time: f64 = (Utc::now().naive_utc().timestamp_nanos()
                - entry_time.0.timestamp_nanos()) as f64
                / 1_000_000.00;
            let log = req.state().logger.clone();
            let method = req.method().to_string();
            let path = req.path();
            let query = req.query_string();
            let version = format!{"{:?}", req.version()};
            let status = resp.status().as_u16();
            let size = resp.response_size();
            let addr = req.connection_info().remote().unwrap().to_string();
            info!(log,
                  "[{}] {} {}", status, method.clone(), path;
                  "size" => size,
                  "version" => version,
                  "remote" => addr,
                  "duration" => elapsed_time,
                  "query" => query,
                  "path" => path,
                  "status" => status,
                  "method" => method);
        }
        Finished::Done
    }
}

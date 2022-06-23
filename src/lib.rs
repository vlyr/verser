#![feature(async_closure)]

use serde::Serialize;

pub mod router;
pub use router::Router;

pub mod route;
pub use route::Route;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct Response {
    pub content: String,
}

impl Response {
    pub fn json<T>(data: T) -> Self
        where T: Serialize
    {
        Self {
            content: serde_json::to_string(&data).unwrap()
        }
    }

    pub fn raw_text<T>(data: T) -> Self 
        where T: AsRef<str>
    {
        Self {
            content: data.as_ref().into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[derive(Clone, Serialize)]
    pub struct State {
        num: i32,
    }

    #[tokio::test]
    async fn basic() {
        let state = Arc::new(Mutex::new(State { num: 123 }));

        let mut router = Router::new(state.clone());

        router.get("/hello/world", |req, state| async move {
            Ok(Response::json(state.lock().await.num))
        });

        router.run("127.0.0.1:6795").await.unwrap();
    }
}


#![feature(async_closure)]

pub mod router;
pub use router::Router;

pub mod route;
pub use route::Route;

pub mod core;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Response;
    use serde::Serialize;
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

        router.get("/", async move |req, state| {
            println!("{:#?}", req);

            let mut state = state.lock().await;
            state.num += 1;

            Ok(Response::json(state.num))
        });

        router.middleware(async move |req, _state| {
            println!("{}", req.identifier());
        });

        router.run("127.0.0.1:6795").await.unwrap();
    }
}

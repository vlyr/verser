use crate::core::{Method, Request, Response, RouteHandler};
use std::pin::Pin;

pub struct Route<T> {
    handler: RouteHandler<T>,
    path: String,
    method: Method,
}

impl<T> Route<T>
where
    T: 'static + Send + Sync,
{
    pub fn new<S>(path: S, method: Method, handler: RouteHandler<T>) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            path: path.as_ref().into(),
            method,
            handler,
        }
    }

    pub async fn exec(&self, request: Request, state: T) -> Response {
        let handler = self.handler.as_ref();

        Pin::from(handler(request, state)).await.unwrap()
    }

    // Will return "GET /hello/world", for example
    pub fn identifier(&self) -> String {
        format!("{} {}", self.method.to_string(), self.path())
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}

use crate::core::{Method, Request};
use crate::Response;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub type Handler<T> = Box<dyn Fn(Request, T) -> HandlerResult + Send + Sync>;
pub type HandlerResult = Box<dyn Future<Output = Result<Response, Box<dyn Error>>> + Send>;

pub struct Route<T> {
    handler: Handler<T>,
    path: String,
    method: Method,
}

impl<T> Route<T>
where
    T: 'static + Send + Sync,
{
    pub fn new<S>(path: S, method: Method, handler: Handler<T>) -> Self
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

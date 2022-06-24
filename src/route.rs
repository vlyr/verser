use crate::Response;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        use Method::*;

        match self {
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
        }
        .into()
    }
}

pub type Handler<T> = Box<dyn Fn(String, T) -> HandlerResult + Send + Sync>;
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

    pub async fn exec(&self, request: String, state: T) -> Response {
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

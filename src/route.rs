use crate::Response;
use std::future::Future;
use futures::future::BoxFuture;
//use crate::router::Handler;
use std::pin::Pin;


pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

pub type Handler<T> = Box<dyn Fn(String, T) -> Box<dyn Future<Output = Result<Response, Box<dyn std::error::Error>>>>>;

pub struct Route<T> {
    handler: Handler<T>,
    path: String,
    method: Method,
}

impl<T> Route<T>
    where T: 'static 
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

    pub async fn exec(&self, request: String, state: T) {
        let handler = self.handler.as_ref();

        Pin::from(handler(request, state)).await.unwrap();
    }
}

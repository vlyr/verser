use anyhow::Result;
use std::future::Future;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::collections::BTreeMap;
use crate::Response;
use crate::Route;
use crate::route::{Handler, Method};
use futures::future::BoxFuture;

pub struct Router<T> {
    routes: Vec<Route<T>>,
    state: T,
}

impl<T> Router<T>
where 
    T: Clone + Send + Sync + 'static,
{
    pub fn new(state: T) -> Self {
        Self {
            routes: vec![],
            state,
        }
    }

    pub fn get<S, Fut, H>(&mut self, path: S, handler: H)
    where 
        S: AsRef<str>,
        H: Fn(String, T) -> Fut + 'static,
        Fut: Future<Output = Result<Response, Box<dyn std::error::Error>>> + 'static
    {
        let handler: Handler<T> = Box::new(move |req, state| Box::new(handler(req, state)));
        let route = Route::new(path, Method::Get, handler);
        self.routes.push(route);
    }

    pub async fn run<S>(&self, addr: S) -> Result<()>
        where S: ToSocketAddrs
    {
        let listener = TcpListener::bind(addr).await?;

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(async move {
                        client_event_loop(stream).await.unwrap();
                    });
                }

                Err(_) => break,
            }
        }

        Ok(())
    }
}

async fn client_event_loop(stream: TcpStream) -> Result<()> {
    loop {
    }

    Ok(())
}

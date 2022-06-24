use std::mem;
use std::sync::Arc;

use anyhow::Result;
use std::future::Future;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::{
    core::{Method, Request},
    route::Handler,
    Response, Route,
};

pub struct RouterState<T> {
    routes: Vec<Route<T>>,
}

impl<T> Default for RouterState<T> {
    fn default() -> Self {
        Self { routes: vec![] }
    }
}

pub struct Router<T> {
    router_state: RouterState<T>,
    user_state: T,
}

impl<T> Router<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(user_state: T) -> Self {
        Self {
            router_state: RouterState::default(),
            user_state,
        }
    }

    pub fn get<S, Fut, H>(&mut self, path: S, handler: H)
    where
        S: AsRef<str>,
        H: Fn(Request, T) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Result<Response, Box<dyn std::error::Error>>> + 'static + Send,
    {
        let handler: Handler<T> = Box::new(move |req, state| Box::new(handler(req, state)));
        let route = Route::new(path, Method::Get, handler);

        self.router_state.routes.push(route);
    }

    pub async fn run<S>(&mut self, addr: S) -> Result<()>
    where
        S: ToSocketAddrs + std::fmt::Debug,
    {
        println!("Running server on {:#?}", addr);
        let listener = TcpListener::bind(addr).await?;

        let router_state = Arc::new(mem::take(&mut self.router_state));

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let router_state = Arc::clone(&router_state);
                    let user_state = self.user_state.clone();

                    tokio::spawn(async move {
                        client_handler(stream, router_state, user_state)
                            .await
                            .unwrap();
                    });
                }

                Err(_) => break,
            }
        }

        Ok(())
    }
}

async fn client_handler<T>(
    mut stream: TcpStream,
    router_state: Arc<RouterState<T>>,
    user_state: T,
) -> Result<()>
where
    T: Send + Sync + 'static + Clone,
{
    let mut buf = vec![0; 1024];
    stream.read(&mut buf).await?;

    buf.retain(|byte| *byte != u8::MIN);

    let buffer = String::from_utf8(buf)?;

    if let Some(req) = Request::from_raw(buffer) {
        match router_state
            .routes
            .iter()
            .find(|r| r.identifier() == req.identifier())
        {
            Some(route) => {
                let response = route.exec(req, user_state.clone()).await;
                let resp = "HTTP/1.1 200 OK\r\nContent-Type:text/html;charset=utf-8\r\n\r\n";

                stream
                    .write(format!("{}{}\r\n\r\n", resp, response.content).as_bytes())
                    .await?;
                stream.flush().await?;
            }

            None => {
                let resp = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html;charset=utf-8\r\n\r\n";

                stream.write(resp.as_bytes()).await?;
                stream.flush().await?;
            }
        }
    }

    Ok(())
}

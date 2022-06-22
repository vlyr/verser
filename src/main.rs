use anyhow::Result;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::collections::BTreeMap;

type Handler<T> = fn(String, T);

pub struct Router<T>
{
    routes: BTreeMap<String, fn(String, T) -> ()>,
}

#[derive(Clone)]
pub struct State {

}

impl<T> Router<T>
where 
    T: Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
        }
    }

    pub fn route<S>(&mut self, path: S, handler: Handler<T>)
        where S: AsRef<str>
    {
        self.routes.insert(path.as_ref().to_string(), handler);
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

#[tokio::main]
async fn main() -> Result<()> {
    let mut router: Router<State> = Router::new();

    router.route("/hello/world", |req, state| {

    });

    router.run("127.0.0.1:6795").await?;

    Ok(())
}

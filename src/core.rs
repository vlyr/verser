use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;

pub type RouteHandler<T> = Box<dyn Fn(Request, T) -> RouteHandlerResult + Send + Sync>;
pub type RouteHandlerResult = Box<dyn Future<Output = Result<Response, Box<dyn Error>>> + Send>;

pub type Middleware<T> = Box<dyn Fn(Request, T) -> MiddlewareOutput + Send + Sync>;
pub type MiddlewareOutput = Box<dyn Future<Output = ()> + Send>;

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

impl<T> From<T> for Method
where
    T: AsRef<str>,
{
    fn from(data: T) -> Self {
        use Method::*;
        match data.as_ref() {
            "GET" => Get,
            "POST" => Post,
            "PUT" => Put,
            "DELETE" => Delete,
            _ => panic!("Invalid string provided during construction of Method enum, string provided was {}", data.as_ref()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    path: String,
    body: String,
    headers: HashMap<String, String>,
    parameters: HashMap<String, String>,
}

impl Request {
    pub fn from_raw(data: String) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        let mut lines = data.split("\n");
        let line = lines.next()?;

        let mut header_line = line.splitn(3, " ");

        let method = header_line.next()?;
        let path = header_line.next()?;

        let mut headers = HashMap::new();

        loop {
            let line = match lines.next() {
                Some(line) => line,
                None => break,
            };

            match line.split_once(": ") {
                Some((k, v)) => {
                    headers.insert(k.into(), v.replace("\r", ""));
                }

                None => break,
            }
        }

        let mut body_lines = vec![];

        for line in lines {
            body_lines.push(line);
        }

        Some(Self {
            method: Method::from(method),
            path: path.into(),
            headers,
            parameters: HashMap::new(),
            body: body_lines.join("\r\n"),
        })
    }

    pub fn identifier(&self) -> String {
        format!("{} {}", self.method.to_string(), self.path)
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &String {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub content: String,
}

impl Response {
    pub fn json<T>(data: T) -> Self
    where
        T: Serialize,
    {
        Self {
            content: serde_json::to_string(&data).unwrap(),
        }
    }

    pub fn raw_text<T>(data: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            content: data.as_ref().into(),
        }
    }
}

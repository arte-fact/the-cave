pub mod session;
use std::collections::HashMap;

pub fn html_response(layout: String, content: String, session_id: &str) -> String {
    let mut contents = layout;
    contents = contents.replace("{{ content }}", &content);
    let length = contents.len();

    let headers = [
        "HTTP/1.1 200 OK",
        "Content-Type: text/html; charset=UTF-8",
        "Cache-Control: no-cache",
        &format!("Set-Cookie: session={}", session_id),
        &format!("Content-Length: {}", length),
    ];

    headers.join("\r\n") + "\r\n\r\n" + &contents
}

pub fn text_response(content: String) -> String {
    let length = content.len();
    let headers = [
        "HTTP/1.1 200 OK",
        "Content-Type: text/html; charset=UTF-8",
        &format!("Content-Length: {}", length),
    ];

    headers.join("\r\n") + "\r\n\r\n" + &content
}

#[derive(Debug, Clone)]
pub enum Method {
    Get,
    Post,
    Unhandled,
}

pub fn parse_method(header: &str) -> Method {
    let method = header
        .split_whitespace()
        .nth(0)
        .unwrap_or("/")
        .replace("/", "");

    match method.as_str() {
        "GET" => return Method::Get,
        "POST" => return Method::Post,
        _ => return Method::Unhandled,
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: Vec<String>,
    pub headers: Vec<Header>,
}

impl Request {
    pub fn get_cookie(&self, name: &str) -> Option<String> {
        for header in self.headers.iter() {
            if let Header::Cookie(cookie) = header {
                return cookie.get(name).map(|s| s.to_string())
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum Header {
    Cookie(HashMap<String, String>),
    _Host(String),
}

pub fn parse_cookie(header_line: &str) -> HashMap<String, String> {
    let cookie = header_line.split_whitespace().nth(1).unwrap_or("/");
    let cookie_content = cookie
        .replace("Cookie:", "")
        .split(";")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let mut cookie_map = HashMap::new();
    for c in cookie_content {
        let key_value = c.split("=").collect::<Vec<&str>>();
        if key_value.len() == 2 {
            cookie_map.insert(
                key_value[0].to_string(),
                key_value[1].to_string()
            );
            continue;
        }
    }

    cookie_map
}

pub fn parse_request(http_request: Vec<String>) -> Request {
    let method = parse_method(&http_request[0]);
    let path = parse_path(&http_request[0]);

    let mut headers = vec![];

    for line in http_request.iter() {
        let split = line.split(":");
        match split.clone().nth(0) {
            Some("Cookie") => {
                headers.push(Header::Cookie(parse_cookie(line)));
            },
            _ => (),
        }
    }

    Request {
        method,
        path,
        headers,
    }
}

pub fn parse_path(header: &str) -> Vec<String> {
    let path = header.split_whitespace().nth(1).unwrap_or("/");
    path.split("/").map(|s| s.to_string()).skip(1).collect::<Vec<String>>()
}

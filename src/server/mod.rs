use std::collections::HashMap;
use std::fs;

pub fn parse_post_request_body(http_request: Vec<String>) -> String {
    let body = http_request[http_request.len() - 1].clone();
    body.split("\0").collect::<Vec<&str>>()[0].to_string()
}


pub fn html_response(content: String, session_id: &str) -> String {
    let mut contents = match fs::read_to_string("assets/index.html") {
        Ok(contents) => contents,
        Err(_) => String::from("Error reading index.html"),
    };

    contents = contents.replace("{{ content }}", &content);
    let length = contents.len();

    let headers = [
        "HTTP/1.1 200 OK",
        "Content-Type: text/html; charset=UTF-8",
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
    pub body: String,
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
    pub fn get_host(&self) -> Option<String> {
        for header in self.headers.iter() {
            if let Header::Host(host) = header {
                return Some(host.clone())
            }
        }
        None
    }
}
#[derive(Debug, Clone)]
pub enum Header {
    Cookie(HashMap<String, String>),
    Host(String),
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
    let path = parse_path(&http_request[0]).split("/").map(|s| s.to_string()).collect::<Vec<String>>();

    let mut headers = vec![];

    for line in http_request.iter() {
        let split = line.split(":");
        match split.clone().nth(0) {
            Some("Cookie") => {
                headers.push(Header::Cookie(parse_cookie(line)));
            },
            Some("Host") => {
                headers.push(Header::Host(split.clone().nth(1).unwrap_or("/").trim().to_string()));
            }
            _ => (),
        }
    }

    let body = parse_post_request_body(http_request);

    Request {
        method,
        path,
        headers,
        body,
    }
}

pub fn parse_path(header: &str) -> String {
    let path = header.split_whitespace().nth(1).unwrap_or("/");
    path.replace("/", "")
}

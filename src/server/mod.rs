use std::fs;

pub fn parse_post_request_body(http_request: Vec<String>) -> String {
    let body = http_request[http_request.len() - 1].clone();
    body.split("\0").collect::<Vec<&str>>()[0].to_string()
}

pub fn html_response(content: String) -> String {
    let mut contents = match fs::read_to_string("assets/index.html") {
        Ok(contents) => contents,
        Err(_) => String::from("Error reading index.html"),
    };

    contents = contents.replace("{{ content }}", &content);
    let length = contents.len();

    let headers = [
        "HTTP/1.1 200 OK",
        "Content-Type: text/html; charset=UTF-8",
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

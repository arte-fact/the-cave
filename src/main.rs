mod map;
mod game;
mod server;

use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use self::game::{Action, Game};
use self::server::{html_response, parse_method, parse_post_request_body, text_response, Method};

fn handle_get(game: &mut Game) -> String {
    html_response(game.draw().split("\n").collect::<Vec<&str>>().join("<br>"))
}

fn handle_post(
    http_request: Vec<String>,
    game: &mut Game,
) -> Result<String, Box<dyn std::error::Error>> {
    let key = parse_post_request_body(http_request);
    let action = Action::from_key(key);
    game.handle_key(action);
    Ok(text_response(game.draw()))
}

fn handle_connection(stream: TcpStream, game: &mut Game) -> String {
    let mut buffer = [0; 1024];
    let mut buf_reader = BufReader::new(stream);
    match buf_reader.read(&mut buffer) {
        Err(e) => return format!("HTTP/1.1 500\r\n\r\n{}", e),
        Ok(_) => (),
    }
    let http_request = String::from_utf8_lossy(&buffer[..])
        .split("\r\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let method = parse_method(&http_request[0]);
    let res = match method {
        Method::Get => handle_get(game),
        Method::Post => {
            handle_post(http_request, game).unwrap_or_else(|e| format!("HTTP/1.1 500\r\n\r\n{}", e))
        }
        Method::Unhandled => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
    };
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Error: {}", e),
            Ok(mut stream) => {
                stream.write_all(handle_connection(stream.try_clone()?, &mut game).as_bytes())?
            }
        }
    }
    Ok(())
}

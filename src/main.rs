mod game;
mod map;
mod server;

use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use self::game::{Action, Game};
use self::server::{
    html_response, parse_request, text_response, Method, Request,
};

fn handle_get(game: &Game) -> String {
    game.draw().split("\n").collect::<Vec<&str>>().join("<br>")
}

fn handle_post(request: Request, game: &mut Game) -> Result<String, Box<dyn std::error::Error>> {
    let action = &request.body;
    let action = Action::from_key(action);
    game.handle_key(action);
    Ok(game.draw())
}

fn handle_preview_map(game: &Game) -> String {
    game.preview_map()
}

fn handle_connection(stream: &TcpStream, games: &mut HashMap<String, Game>) -> String {
    let mut buffer = [0; 1024];
    let mut buf_reader = BufReader::new(stream);
    match buf_reader.read(&mut buffer) {
        Err(e) => return format!("HTTP/1.1 500\r\n\r\n{}", e),
        Ok(_) => (),
    }

    let request = parse_request(
        String::from_utf8(buffer.to_vec())
            .unwrap_or_default()
            .split("\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );

    let path = match request.path.first() {
        Some(p) => p.clone(),
        None => return "HTTP/1.1 404\r\n\r\n".to_string(),
    };

    let host = match request.get_host() {
        Some(h) => h,
        None => return "HTTP/1.1 500\r\n\r\nHost header not found".to_string(),
    };

    let session_id = request.get_cookie("session").unwrap_or_else(|| {
        let session_id = format!("{:x}", rand::random::<u128>());
        session_id
    });

    if !games.contains_key(&session_id) {
        games.insert(session_id.clone(), Game::new());
    }
    let mut game = games.get(&session_id).unwrap().clone();

    let res = match request.method {
        Method::Get => match path.as_str() {
            "" => html_response(handle_get(&game), &host, &session_id),
            "map" => html_response(handle_preview_map(&game), &host, &session_id),
            _ => return "HTTP/1.1 404\r\n\r\n".to_string(),
        },
        Method::Post => {
            let res = handle_post(request, &mut game)
                .unwrap_or_else(|e| format!("HTTP/1.1 500\r\n\r\n{}", e));
            games.insert(session_id.clone(), game);
            text_response(res, &host, &session_id)
        }
        Method::Unhandled => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
    };
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    let games: Arc<Mutex<HashMap<String, Game>>> = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        let games = games.clone();
        match stream {
            Err(e) => println!("Error: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    let mut stream = match stream.try_clone() {
                        Ok(s) => s,
                        Err(e) => {
                            println!("Error: {}", e);
                            return;
                        }
                    };
                    let mut games = match games.lock() {
                        Ok(g) => g,
                        Err(e) => {
                            println!("Error: {}", e);
                            return;
                        }
                    };
                    stream
                        .write_all(handle_connection(&stream, &mut games).as_bytes())
                        .unwrap();
                });
            }
        }
    }
    Ok(())
}

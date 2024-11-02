mod game;
mod map;
mod server;

use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};


use self::game::{Action, Game};
use self::server::{html_response, parse_request, set_cookie_and_redirect, text_response, Method, Request};



fn handle_get(game: &Game) -> String {
    html_response(
        game.draw().split("\n").collect::<Vec<&str>>().join("<br>"),
    )
}

fn handle_post(request: Request, game: &mut Game) -> Result<String, Box<dyn std::error::Error>> {
    let action = &request.body;
    let action = Action::from_key(action);
    game.handle_key(action);
    Ok(text_response(game.draw()))
}

fn handle_preview_map(game: &Game) -> String {
    html_response(game.preview_map())
}

fn handle_connection(stream: TcpStream, games: &mut HashMap<String, Game>) -> String {
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

    let session_id = match request.get_cookie("session") {
        Some(id) => id.to_string(),
        None => {
            let session_id = format!("{:?}", std::time::SystemTime::now());
            return set_cookie_and_redirect(&session_id);
            
        }
    };

    if !games.contains_key(&session_id) {
        games.insert(session_id.clone(), Game::new());
    }
    let mut game = games.get(&session_id).unwrap().clone();

    let res = match request.method {
        Method::Get => match path.as_str() {
            "" => handle_get(&game),
            "map" => handle_preview_map(&game),
            _ => return "HTTP/1.1 404\r\n\r\n".to_string(),
        },
        Method::Post => {
            let res = handle_post(request, &mut game).unwrap_or_else(|e| format!("HTTP/1.1 500\r\n\r\n{}", e));
            games.insert(session_id, game);
            res
        }
        Method::Unhandled => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
    };
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:7777")?;
    let mut games: HashMap<String, Game> = HashMap::new();
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Error: {}", e),
            Ok(mut stream) => {
                stream.write_all(handle_connection(stream.try_clone()?, &mut games).as_bytes())?
            }
        }
    }
    Ok(())
}

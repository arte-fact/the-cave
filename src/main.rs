mod game;
mod map;
mod server;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GAMES: Arc<Mutex<HashMap<String, Game>>> = Arc::new(Mutex::new(HashMap::new()));
}

use self::game::{Action, Game};
use self::server::{html_response, parse_request, set_cookie_and_redirect, text_response, Method, Request};

pub fn get_game_or_new(game_id: &str) -> Game {
    let mut game_map = GAMES.lock().expect("Could not lock game map");
    let game = game_map.get_mut(game_id).map(|g| g.clone());
    match game {
        Some(g) => g,
        None => {
            let new_game = Game::new();
            game_map.insert(game_id.to_string(), new_game.clone());
            new_game
        }
    }
}

pub fn set_game(game_id: &str, game: Game) {
    let mut game_map = GAMES.lock().expect("Could not lock game map");
    game_map.insert(game_id.to_string(), game);
}

fn handle_get(session_id: &str) -> String {
    let game = get_game_or_new(&session_id);
    html_response(
        game.draw().split("\n").collect::<Vec<&str>>().join("<br>"),
    )
}

fn handle_post(request: Request, session_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut game = get_game_or_new(session_id);
    let action = &request.body;
    let action = Action::from_key(action);
    game.handle_key(action);
    set_game(session_id, game.clone());
    Ok(text_response(game.draw()))
}

fn handle_preview_map(session_id: &str) -> String {
    html_response(get_game_or_new(session_id).preview_map())
}

fn handle_connection(stream: TcpStream) -> String {
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

    let res = match request.method {
        Method::Get => match path.as_str() {
            "" => handle_get(&session_id),
            "map" => handle_preview_map(&session_id),
            _ => return "HTTP/1.1 404\r\n\r\n".to_string(),
        },
        Method::Post => {
            handle_post(request, &session_id).unwrap_or_else(|e| format!("HTTP/1.1 500\r\n\r\n{}", e))
        }
        Method::Unhandled => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
    };
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Error: {}", e),
            Ok(mut stream) => {
                stream.write_all(handle_connection(stream.try_clone()?).as_bytes())?
            }
        }
    }
    Ok(())
}

mod assets;
mod html;
mod pages;
mod game;
mod map;
mod server;
mod tile;

use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use self::game::action::Action;
use self::game::Game;
use self::pages::game::game_page;
use self::pages::map::map_page;
use self::server::{html_response, parse_request, text_response, Method, Request};

fn handle_get(game: &Game) -> String {
    game.draw().split("\n").collect::<Vec<&str>>().join("<br>")
}

fn handle_key(request: &Request, game: &mut Game) -> Result<String, Box<dyn std::error::Error>> {
    let action = Action::from_key(&request.path[1]);
    if action == Some(Action::Unhandled) || action == None {
        return Err("HTTP/1.1 400\r\n\r\nBad Request".into());
    }
    game.handle_key(action.unwrap());
    Ok(game.draw())
}

fn handle_preview_map(game: &Game) -> String {
    game.preview_map()
}

fn handle_connection(
    stream: &TcpStream,
    sessions: &Vec<String>,
    games: &Vec<Game>,
) -> Result<(String, String, Game), String> {
    let mut buffer = [0; 1024];
    let mut buf_reader = BufReader::new(stream);
    buf_reader
        .read(&mut buffer)
        .map_err(|e| format!("HTTP/1.1 500\r\n\r\n{}", e))?;

    let request = parse_request(
        String::from_utf8(buffer.to_vec())
            .map_err(|e| format!("HTTP/1.1 500\r\n\r\n{}", e))?
            .split("\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );

    let path = match request.path.first() {
        Some(p) => p.clone(),
        None => return Err("HTTP/1.1 404\r\n\r\nNot Found".to_string()),
    };

    let session_id = request.get_cookie("session").unwrap_or_else(|| {
        let session_id = format!("{:x}", rand::random::<u128>());
        session_id
    });

    let session_id_2 = session_id.clone();
    let game_session = sessions
        .iter()
        .enumerate()
        .find(|(_, s)| s == &&session_id_2);

    let mut game: Option<Game> = None;
    if game_session.is_none() {
        game = Some(Game::new());
    }
    if game == None {
        game = Some(games[game_session.unwrap().0].clone());
    }
    let mut game = game.unwrap();

    let res = match request.method {
        Method::Get => match path.as_str() {
            "" => html_response(game_page(), handle_get(&game), &session_id),
            "map" => html_response(map_page(), handle_preview_map(&game), &session_id),
            "key" => match handle_key(&request, &mut game) {
                Ok(r) => text_response(r),
                Err(e) => e.to_string(),
            },
            _ => return Err("HTTP/1.1 404\r\n\r\nNot Found".to_string()),
        },
        Method::Unhandled => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
        Method::Post => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
    };
    Ok((res, session_id, game))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:9999")?;
    let games: Arc<Mutex<Vec<Game>>> = Arc::new(Mutex::new(vec![]));
    let sessions: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    listener.incoming().into_iter().for_each(|stream| {
        match stream {
            Err(e) => println!("Error: {}", e),
            Ok(stream) => {
                let mut stream = match stream.try_clone() {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Error: {}", e);
                        return;
                    }
                };

                let games = Arc::clone(&games);
                let sessions = Arc::clone(&sessions);
                tokio::spawn(async move {
                    let games = games.lock();
                    let sessions = sessions.lock();
                    if games.is_err() || sessions.is_err() {
                        stream
                            .write_all("HTTP/1.1 500\r\n\r\nInternal Server Error".as_bytes())
                            .unwrap();
                        return;
                    }
                    let mut sessions = sessions.unwrap();
                    let mut games = games.unwrap();

                    let (res, session, game) =
                        match handle_connection(&stream, &sessions, &games) {
                            Ok(r) => r,
                            Err(e) => {
                                stream.write_all(e.as_bytes()).unwrap();
                                return;
                            }
                        };

                    let game_session = sessions.iter().enumerate().find(|(_, s)| *s == &session);
                    if game_session.is_none() {
                        sessions.push(session.clone());
                        games.push(game);
                    } else {
                        games[game_session.unwrap().0] = game;
                    }

                    stream.write_all(res.as_bytes()).unwrap();
                });
            }
        }
    });
    Ok(())
}

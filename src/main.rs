mod game;
mod map;
mod server;

use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

use self::game::{Action, Game};
use self::server::{html_response, parse_request, text_response, Method, Request};

fn handle_get(game: &Game) -> String {
    game.draw().split("\n").collect::<Vec<&str>>().join("<br>")
}

fn handle_post(request: Request, game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    let action = &request.body;
    let action = Action::from_key(action);
    game.handle_key(action);
    Ok(())
}

fn handle_preview_map(game: &Game) -> String {
    game.preview_map()
}

fn handle_connection(stream: &TcpStream, sessions: &Vec<String>, games: &Vec<Game>) -> Result<(String, String, Game), String> {
    let mut buffer = [0; 2048];
    let mut buf_reader = BufReader::new(stream);
    buf_reader.read(&mut buffer).map_err(|e| format!("HTTP/1.1 500\r\n\r\n{}", e))?;
    println!("Request: {:?}", String::from_utf8(buffer.to_vec()).unwrap());

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
    let game_session = sessions.iter().enumerate().find(|(_, s)| s == &&session_id_2);

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
            "" => html_response(handle_get(&game), &session_id),
            "map" => html_response(handle_preview_map(&game), &session_id),
            _ => return Err("HTTP/1.1 404\r\n\r\nNot Found".to_string()),
        },
        Method::Post => {
            handle_post(request, &mut game).map_err(|e| format!("HTTP/1.1 500\r\n\r\n{}", e))?;
            text_response(game.draw())
        }
        Method::Unhandled => "HTTP/1.1 405\r\n\r\nMethod Not Allowed".to_string(),
    };
    Ok((res, session_id, game))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:9999")?;
    let mut games: Vec<Game> = vec![];
    let mut sessions: Vec<String> = vec![];
    for stream in listener.incoming() {
        match stream {
            Err(e) => println!("Error: {}", e),
            Ok(stream) => {
                let mut stream = match stream.try_clone() {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(Box::new(e));
                    }
                };
                let (res, session, game) = match handle_connection(&stream, &sessions, &games) {
                    Ok(r) => r,
                    Err(e) => {
                        stream.write_all(e.as_bytes())?;
                        continue;
                    }
                };

                let game_session = sessions.iter().enumerate().find(|(_, s)| *s == &session);
                if game_session.is_none() {
                    sessions.push(session.clone());
                    games.push(game);
                } else {
                    games[game_session.unwrap().0] = game;
                }

                stream.write_all(res.as_bytes())?;
            }
        }
    }
    Ok(())
}

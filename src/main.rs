use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
// use nom::character::streaming::u8;
use std::fs::File;
use std::env;
use std::path::PathBuf;


const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
// const CREATED_RESPONSE: &str = "HTTP/1.1 201 Created\r\n";
// const CONTENT_TYPE: &str = "Content-Type: text/plain\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
// const ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
// const OCTET_STREAM: &str = "Content-Type: application/octet-stream\r\n";

// enum Methods {
//     GET,
//     POST,
//     DELETE,
// }

// enum HttpResponseLine {
//     Okk(&str),
//     // Created,
//     // NotFound,
//     // Error,
// }

// impl HttpResponseLine {
//     fn to_string(&self) -> &str {
//         match self {
//             HttpResponseLine::Okk => "HTTP/1.1 200 OK\r\n",
//             // Created => b"HTTP/1.1 201 Created\r\n",
//             // NotFound => b"HTTP/1.1 404 NOT FOUND\r\n\r\n",
//             // Error => b"HTTP/1.1 500 Internal Server Error\r\n\r\n",
//         }
//     }
// }

#[allow(dead_code)]
struct Request {
    request_line:   String,
    method:         String,
    uri:            String,
    http_version:   String,
    headers:        HashMap<String, String>,
    // body:           String,
}

fn process_request(mut buffer: BufReader<&mut TcpStream>) -> std::io::Result<Request> {

    let mut headers: HashMap<String, String> = HashMap::new();
    let mut request_line = String::new();
    
    buffer.read_line(&mut request_line)?;
    let request_elements: Vec<&str> = request_line.trim().split_whitespace().collect();

    loop {
        let mut line = String::new();
        buffer.read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        
        let trimmed_line = line.trim();
        let mut pair = trimmed_line.splitn(2, ':');
        if let Some(key) = pair.next() {
            if let Some(value) = pair.next() {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    Ok(Request {
        request_line: request_line.clone(),
        method: request_elements[0].to_string(),
        uri: request_elements[1].to_string(),
        http_version: request_elements[2].to_string(),
        headers,
    })
}

fn read_file(path_to_file: &str) -> std::io::Result<String> {

    println!("In read_file");
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    println!("Exe_path is : {}", exe_path.display());
    // Get the parent directory of the executable
    let mut parent_dir = exe_path.parent().expect("Failed to get parent directory of executable").to_path_buf();
    println!("Parent_dir is : {}", parent_dir.display());
    
    parent_dir.push(path_to_file);
    let mut file = File::open(parent_dir)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn get_body(request: &Request, path: &str) -> Option<String> {

    match path {
        "/user-agent" => {
            if let Some(agent) = request.headers.get("User-Agent") {
                Some(agent.to_string())
            } else {
                None
            }
        }
        "/echo" => {
            if request.uri.starts_with(path) {
                let body = &request.uri[path.len() + 1..];
                Some(body.to_string())
            } else {
                None
            }
        }
        "/" => {
            Some("".to_string())
        }
        _ => match read_file(&request.uri) {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }
}

fn get_route(uri: &str) -> Option<String> {
    
    if let Some(s1) = uri.find("/") {
        if let Some(s2) = uri[s1 + 1..].find("/") {
            return Some(uri[s1..s2 + 1].to_string());
        } else {
            return Some(uri[s1..].to_string());
        }
    } else {
        return None;
    }
}

fn build_response(mut _pending_request: Request, mut stream: TcpStream) -> std::io::Result<()>{

    let uri = _pending_request.uri.clone();
    let path = get_route(&uri).unwrap();
    let mut _s: &str = "/";
    
    println!("Path recupere : {}", path);

    let response_line = OK_RESPONSE;//              Must use Rust enum instead of global const
    let content = "Content-Type: text/plain\r\n".to_string();
    
    match get_body(&_pending_request, &path) {
        Some(agent) => {
            println!("agent is : {}", agent);
            let body = agent;
            let l = body.len();
            let length = format!("Content-Length: {}\r\n", l);
            let headers = format!("{}{}\r\n", content, length);
            let response = format!("{}{}{}\r\n", response_line, headers, body);
            stream.write(response.as_bytes())?;
            println!("RESPONSE::\n\n{}", response);
        }
        None => {
            stream.write_all(NOT_FOUND_RESPONSE.as_bytes())?;
            println!("404 Sent.");
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {

    println!("Handling client...");
    let reader = BufReader::new(&mut stream);

    match process_request(reader) {
        Ok(request) => {
            println!("Client created !");
            build_response(request, stream);
        }
        Err(e) => {
            println!("Error creating client : {}", e);
        }
    }
    Ok(())
}

fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let _res = handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
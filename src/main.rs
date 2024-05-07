use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use nom::character::streaming::u8;

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const CREATED_RESPONSE: &str = "HTTP/1.1 201 Created\r\n";
const CONTENT_TYPE: &str = "Content-Type: text/plain\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const ERROR_RESPONSE: &str = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
const OCTET_STREAM: &str = "Content-Type: application/octet-stream\r\n";

enum Methods {
    GET,
    POST,
    DELETE,
}

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

struct Request {
    request_line:   String,
    method:         String,
    uri:            String,
    http_version:   String,
    headers:        HashMap<String, String>,
    // body:           String,
}

fn create_request(mut buffer: BufReader<&mut TcpStream>) -> std::io::Result<Request> {

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

fn build_response(mut _pending_request: Request, mut stream: TcpStream) -> std::io::Result<()>{

    let path = _pending_request.uri;
    let mut s: &str = "/";
    
    if path.contains("/echo/") {//            Ugly way to manage uri, must change
        s = &path[..6];
    }
    
    if s == "/echo/" {
        let response_line = OK_RESPONSE;//              Must use Rust enum instead of global const
        let content = "Content-Type: text/plain\r\n".to_string();
        
        let body: String = path.strip_prefix("/echo/").unwrap().to_string();
        let l = body.len();
        let length = format!("Content-Length: {}\r\n", l);
        let headers = format!("{}{}\r\n", content, length);
        let response = format!("{}{}{}\r\n", response_line, headers, body);
        stream.write(response.as_bytes())?;
        println!("RESPONSE::\n\n{}", response);
    } else if path == "/" {
        stream.write_all(OK_RESPONSE.as_bytes())?;
        stream.write_all("\r\n".as_bytes())?;
        println!("200 sent.");
    } else {
        stream.write_all(NOT_FOUND_RESPONSE.as_bytes())?;
        println!("404 Sent.");
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {

    println!("Handling client...");
    let reader = BufReader::new(&mut stream);

    match create_request(reader) {
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
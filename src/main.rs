use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{BufRead, BufReader};

struct Client {
    request_line: String,
    method:       String,
}

fn extract_path(request: String) -> Option<String> {

    println!("Request is :\n{}", request);
    if let Some(start) = request.find(" ") {
        println!("First match found");
        if let Some(end) = request[start + 1..].find(" ") {
            println!("Second match found");
            let path = &request[start + 1..start + 1 +end];
            println!("PATH is : {}, where start is {} and end is {}", path, start, end);
            return Some(path.to_string());
        } else {
            println!("Second match not found");
        }
    } else {
        println!("First match not found");
    }
    None
}


fn  create_client(mut buffer: BufReader<&mut TcpStream>) -> std::io::Result<Client> {

    let mut buf_line = String::new();
    buffer.read_line(&mut buf_line)?;

    let mut buf_method = Vec::new();
    buffer.read_until(b' ', &mut buf_method)?;


    let method_string = String::from_utf8_lossy(&buf_method).trim().to_string();

    Ok(Client {
        request_line:    buf_line,
        method:         method_string,
    })
}



fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {

    println!("Handling client...");
    let reader = BufReader::new(&mut stream);

    match create_client(reader) {
        Ok(client) => {
            println!("Client created !");
            let path = extract_path(client.request_line);
            match path {
                Some(path) => {
                    println!("Extracted path: {}", path);
                    // if path == "/" {
                    //     // stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
                    //     // println!("200 Sent.");
                    // } else {
                    //     stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
                    //     println!("404 Sent.");
                    // }
                }
                None => {
                    println!("Failed !");
                }
            }
        }
        Err(e) => {
            println!("Error create client : {}", e);
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

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::time::Duration;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (filename, status_line) = if buffer.starts_with(get) {
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(&response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("{}", response);

}

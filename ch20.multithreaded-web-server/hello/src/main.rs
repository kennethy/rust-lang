use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::{io::{prelude::*, BufReader}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        // represents a single open connection
        let stream = stream.unwrap();

        // use a pool instead to prevent spawning unlimited amount of threads
        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("shutting it down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next() // only interested in the first item
        .unwrap() // takes care of the `Option`
        .unwrap(); // takes care of the `Result`

    // match does not do automatic referencing and dereferencing
    // like the equality method does: let x = if a == b { ...
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write(response.as_bytes()).unwrap();
}

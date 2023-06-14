use chrono::offset::Local;
use hello::ThreadPool;
use std::{
    fs,
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let ip = "localhost";
    let port = 7878;
    let pool = ThreadPool::new(10);
    let host = get_host(&ip, port);
    let listener = TcpListener::bind(host).unwrap();

    print_server_info(ip, port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            for i in 0..5 {
                println!("sleep {}th second.", i+1);
                thread::sleep(Duration::from_secs(1));
            }
            ("HTTP/1.1 200 OK", "sleep.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
        
    println!("status_line: {}, filename: {}", status_line, filename);
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_host(ip: &str, port: u32) -> String {
    format!("{}:{}", ip, port)
}

fn get_full_host(ip: &str, port: u32) -> String {
    format!("http://{}:{}/", ip, port)
}

/// > RUST_VERSION=1.58.1 cargo run
/// warning: unused manifest key: package.rust_version
///     Finished dev [unoptimized + debuginfo] target(s) in 0.02s
///      Running `target/debug/hello`
/// Rust version: "1.58.1"
/// packeage version: 0.1.0
/// 2023-06-14 21:57:52
/// Starting development server http://localhost:7878/
/// Quit the server with CONTROL-C.
fn print_server_info(ip: &str, port: u32) {
    let host = get_full_host(ip, port);
    let now = Local::now();
    let format_time = now.format("%Y-%m-%d %H:%M:%S");

    print_rust_env();
    println!("{}", format_time);
    println!("Starting development server {}", host);
    println!("Quit the server with CONTROL-C.");

}

fn print_rust_env() {
    let rust_version = env::var("RUST_VERSION");
    match rust_version {
        Ok(rv) => println!("Rust version: {:?}", rv),
        _ => (),
    }
    println!("packeage version: {}", env!("CARGO_PKG_VERSION"));
}

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, SystemTime},
};

use webserver::ThreadPool;

fn main() {
    let now = SystemTime::now();
    print!("{:?} run\n", now);
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool: ThreadPool = ThreadPool::new(4);

    for stream in listener.incoming().take(1000) {
        let stream = stream.unwrap();
        let now = SystemTime::now();
        print!("{:?} coming\n", now);
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    /*let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    println!("Request: {:#?}", http_request);*/
    
    /*let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };*/
    let (status_line, filename) = match &request_line[..] {
        "GET /cat.jpg HTTP/1.1" => ("HTTP/1.1 200 OK", "cat.jpg"),
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            print!("going to sleep\n");
            thread::sleep(Duration::from_secs(5));
            print!("going over\n");
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    match &filename[..] {
        "cat.jpg" => {
            println!("cat jpg.");
            let response =
                format!("{status_line}\r\n\r\n\r\n");
        
            stream.write_all(response.as_bytes()).unwrap();
        }
        _ => {
            let contents = fs::read_to_string(filename).unwrap();
            let length = contents.len();
        
            let response =
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        
            stream.write_all(response.as_bytes()).unwrap();
        },
    }

}

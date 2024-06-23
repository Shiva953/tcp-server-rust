use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap(); //Returns Err idf unavailable port/permission denied
    let pool = ThreadPool::new(4); // Creating a New 4 thread ThreadPool, each stream would be processed in a thread choosen from this pool
    for stream in listener.incoming() { //listening for the connection
        let stream = stream.unwrap();

        pool.execute(|| { //handling each stream in a thread, to concurrently handle max 4 reqs
            handle_connection(stream);
        }); // every req/res happens in a new thread
        // println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); //reading buffered data from the tcp stream
    // now, we take the individual request and include its parameters(in vector)
    // let http_request: Vec<_> = buf_reader
    //     .lines() //taking req parameters as lines
    //     .map(|result| result.unwrap()) //if err in any line, it panics
    //     .take_while(|line| !line.is_empty()) //removing the empty lines
    //     .collect(); //finally forming a vector
    let request_line = buf_reader.lines().next().unwrap().unwrap(); //taking the FIRST LINE(PARAM) FROM THE HTTP REQUEST,which would be the path after 127.0.0.1/

    let (status_line, filename) = match &request_line[..] { //checking if route matches the / route
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "front.html"),
        "GET /sleep HTTP/1.1" => {
            //slowing down the req(res after 5 secs)
            thread::sleep(Duration::from_secs(5)); //blocks the main thread for 5secs, then returns the html
            ("HTTP/1.1 200 OK", "front.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap(); ////writes the buffered response to the TCP stream(sends res back to the client on successful req)
    
    //Improving Throughput with a Thread Pool
    // A thread pool is a group of spawned threads that are waiting and ready to handle a task.
    // When the program receives a new task, it assigns one of the threads in the pool to the task, 
    // and that thread will process the task. The remaining threads in the pool are available to handle any other tasks
    // that come in while the first thread is processing. When the first thread is done processing its task,
    // it’s returned to the pool of idle threads, 
    // ready to handle a new task. A thread pool allows you to process connections concurrently, 
    // increasing the throughput of your server.

}
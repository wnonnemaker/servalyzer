use std::{
    env,
    io::{copy, prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let recport = &args[1];
    let forport = &args[2];


    let mut recportstring: String = "localhost:".to_owned();
    recportstring.push_str(&recport);

    let mut forportstring: String = "localhost:".to_owned();
    forportstring.push_str(&forport);


    println!("Listening for requests to port {recportstring}");

    let listener = TcpListener::bind(recportstring).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let sender = TcpStream::connect(&forportstring).unwrap();
        println!("Connection established");
        //forward_to_port(stream, sender);
        forward_to_port(stream, sender);
    }
}

fn forward_to_port(mut browserme: TcpStream, mut meserver: TcpStream) {
    let buf_reader = BufReader::new(&browserme);
    for line in buf_reader.lines() {
        let current_line = line.unwrap();
        if current_line == "" { break; }
        let written_line = format!("{current_line}\r\n");
        meserver.write_all(written_line.as_bytes()).unwrap();
    }
    meserver.write_all(b"\r\n");
    meserver.flush().unwrap();

    println!("Reading from vite now"); 
    let mut vite_reader = BufReader::new(&meserver);
    let mut content_length: usize = 100000;
    let mut body: Vec<String> = Vec::new();
    let mut counter: usize = 0; 
    let mut inbody = false;
    let mut currentline = "".to_string();
    while let Ok(size) = vite_reader.read_line(&mut currentline) {
        println!("{currentline:?}");
        if inbody { counter += size; }
        let trimmedline = currentline.trim();
        if trimmedline == "" { inbody=true; } 
        if trimmedline.to_lowercase().starts_with("content-length") {
            let parts: Vec<&str> = trimmedline.split(":").collect();
            if parts.len() == 2 {
                content_length = parts[1].trim().parse().unwrap_or(0);
            }
        }
        println!("{counter}");
        body.push(currentline.clone());
        if counter >= content_length {
            println!("We escaped the loop");
            break;
        }
        currentline.clear();
    }
    println!("We escaped the loop");
    
    //browserme.write_all(body.as_slice()).unwrap();
    for line in &body{
        browserme.write_all(line.as_bytes()).unwrap();
    }
    


    browserme.flush().unwrap();

   /*for line in vite_reader.lines() {
        let current_line = line.unwrap();
        if current_line.to_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = current_line.split(':').collect();
            if parts.len() == 2 {
                content_length = parts[1].trim().parse().unwrap_or(0);
            }
        }

        let written_line = format!("{current_line}\r\n");
        println!("{written_line}");
        browserme.write_all(written_line.as_bytes()).unwrap();
        browserme.flush().unwrap(); 
        if current_line.is_empty() {
            break;
        }
    }
    browserme.write_all(b"\r\n");
    if content_length > 0 {
        let mut body = vec![0; content_length];
        vite_reader.read_exact(&mut body).unwrap();
        browserme.write_all(&body);
    }
    browserme.flush().unwrap(); 
    */
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}


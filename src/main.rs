use std::{
    io::{BufReader, Read, Write},
    net::{SocketAddr, TcpListener},
};

const END_OF_LINE: &str = "\r\n";

fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    // The TCP listener for listening to incoming requests
    let listener = TcpListener::bind(address).unwrap();

    println!("Listening to requests: {address}");

    // We are now listening to any incoming request
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        // For faster/more efficient reading of stream
        let buf_reader = BufReader::new(stream.try_clone().unwrap());

        // Our buffer for reading any data sent by the client, aka the HTTP request
        let mut http_request = String::new();

        // We read byte by byte and we push each character into the buffer
        for byte in buf_reader.bytes() {
            let byte = byte.unwrap();
            let char = byte as char;
            http_request.push(char);

            // If character is \n and the request ends with \r\n\r\n, we stop reading
            if char == '\n' && http_request.ends_with("\r\n\r\n") {
                break;
            }
        }
        println!("{http_request:?}");

        // The body of the HTTP response
        let message_body = "uwu".to_string();
        // The response status
        let status = b"HTTP/1.1 200 OK\r\n";
        // The response headers
        let headers = [
            "Content-Type: text/html".into(),
            format!("Content-Length: {}", message_body.len()),
        ];

        // The response buffer
        let mut response = Vec::new();
        // We write the status in this buffer/array of u8s
        response.write_all(status).unwrap();
        // Now the headers
        let header_string = headers.join("\r\n") + END_OF_LINE;
        response.write_all(header_string.as_bytes()).unwrap();
        // The mandatory empty line separating the headers and the message body
        response.write_all(END_OF_LINE.as_bytes()).unwrap();
        // And now we write the message body
        response.write_all(message_body.as_bytes()).unwrap();

        // We write (or send) the entire response to the stream
        stream.write_all(&response).unwrap();
    }
}

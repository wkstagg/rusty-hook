// ...existing code...
// From Copilot: top-level modules - these bring other source files into scope.
// From Copilot: NOTE: there is a likely typo here: `mod contants;` should probably be `mod constants;`
// From Copilot: If the module name and the `use` below don't match, the code will not compile.
mod cli;
mod contants;
use cli::get_arguments;
// From Copilot: we import two constants from a `constants` module: DEFAULT_PORT and PROTOCOL_STRING
use constants::{DEFAULT_PORT, PROTOCOL_STRING};
use std::io::{Read, Write};
use std::net::TcpStream;

// From Copilot: parse_url takes a URL string and returns a tuple:
// From Copilot: (protocol_string, hostname, path, socket_address)
// From Copilot: protocol_string is looked up in a map (PROTOCOL_STRING) that converts proto -> "HTTP/1.1" etc.
// From Copilot: hostname is the host part, path is the path part (everything after the first '/'),
// From Copilot: socket_address is "host:port" ready to be passed to TcpStream::connect
fn parse_url(url: &str) -> (&str, &str, &str, String) {
    let (protocol, rest) = url.split_once("://").unwrap();
    let (temp_hostname, pathname) = rest.split_once("/").unwrap();
    let (hostname, port) = if temp_hostname.contains(":") {
        let temp_hostname.split_once(":").expect("Invalid hostname")
    } else {
        (temp_hostname, DEFAULT_PORT.to_string())
    };
    let socket_address = format!("{}:{}", hostname, port);
    let protocol_str = PROTOCOL_STRING
        .get(protocol)
        .expect("Unsupported protocol")
        .to_string();  // Not in original codebase
    (protocol_str, hostname, pathname, socket_address)

}

// From Copilot: populate_get_request builds the raw HTTP request string.
// From Copilot: Parameters:
// From Copilot: - protocol: protocol string like "HTTP/1.1"
// From Copilot: - host: hostname for the Host header
// From Copilot: - path: path portion to request (no leading slash here, caller uses "/{path}")
// From Copilot: - data: optional request body (used for POST/PUT)
// From Copilot: - method: optional method (defaults to GET)
// From Copilot: - headers: extra header lines (each element should be a header string like "X-Header: val\r\n")
// From Copilot: The function returns the full request as a single String.
fn populate_get_request(
    protocol: &str,
    host: &str,
    path: &str,
    data: Option<&String>
    method: Option<&String>
    headers: Vec<&str>,
) -> String {
    let default_method = String::from("GET");
    let method = method.unwrap_or(&default_method);
    let mut res = String::new();
    // From Copilot: Start line, e.g. "GET /path HTTP/1.1\r\n"
    res += &format!("{} /{} {}\r\n", method, path, protocol);
    // From Copilot: Required Host header
    res += &format!("Host: {}\r\n", host);
    // From Copilot: Some common headers
    res += "Accept: */*\r\n";
    res += "Connection: close\r\n";
    
    if method == "POST" || method == "PUT" {
        if headers.len() > 0 {
            for head in headers {
                // From Copilot: caller-provided headers should already include trailing \r\n
                res += head;
            }
            res += "\r\n";
        } else {
            // From Copilot: default Content-Type if none provided
            res += "Content-Type: application/json\r\n"
        }
        if let Some(data_str) = data {
            // From Copilot: Add Content-Length and the body
            let data_bytes = data_str.as_bytes();
            res += &format!("Content-Length: {}\r\n\r\n", data_bytes.len());
            res += data_str;
            res += "\r\n";
        }
    }

    // From Copilot: Ensure request ends with an extra CRLF (separates headers from body; if no body, ends the request)
    res += "\r\n";
    res
}

// From Copilot: parse_resp splits the raw response into header and body on the first CRLFCRLF sequence.
// From Copilot: Returns (&str header, &str body). This will panic if the separator is missing.
fn parse_resp(resp: &str) -> (&str, &str) {
    let (response_header, response_data) = resp.split_once("\r\n\r\n").unwrap();
    (response_header, response_data)
}

fn main() {
    let matches = get_arguments();

    // From Copilot: argument matching - check flags and get values from clap (or similar)
    let verbose_enabled = matches.contains_id("verbose") && matches.get_flag("verbose");
    let url = matches.get_one::<String>("url").unwrap();
    let data = matches.get_one::<String>("data");
    let method = matches.get_one::<String>("x-method");
    let headers: Vec<&str> = matches
        .get_many::<String>("headers")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    // From Copilot: parse the provided URL into components
    let (protocol, host, path, socket_address) = parse_url(url);
    // From Copilot: build request buffer string
    let buffer_str = populate_get_request(
        protocol,
        host,
        path,
        data,
        method,
        headers,
    );

    // From Copilot: Open a TCP connection to the socket address (host:port)
    let tcp_socket = TcpStream::connect(socket_address).expect("Could not connect to server");
    match tcp_socket {
        // From Copilot: TcpStream::connect returns a Result; this match expects Ok(stream)
        Ok(mut stream) => {
            stream
                .write_all(buffer_str.as_bytes())
                .expect("Failed to write to socket");

            let mut buffer = Vec::new();
            stream
                .read_to_end(&mut buffer)
                .expect("Failed to read from socket");

            // From Copilot: Convert bytes to string (lossy conversion to avoid invalid UTF-8 panics)
            let response_str = String::from_utf8_lossy(&buffer);
            let (response_header, response_data) = parse_resp(&response_str);

            if verbose_enabled {
                // From Copilot: If verbose, print the request lines prefixed with '>'
                let lines = buffer_str.lines();
                for line in lines {
                    println!("> {}", line);
                }
            }
            // From Copilot: Print only the response body to stdout
            println!("{}", response_data);
        }
        Err(e) => {
            // From Copilot: Print connection error to stderr
            eprintln!("Error: {}", e);
        }
    }
}
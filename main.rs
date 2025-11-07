mod cli;
mod contants;
use cli::get_arguments;
use constants::{DEFAULT_PORT, PROTOCOL_STRING};
use std::io::{Read, Write};
use std::net::TcpStream;

fn parse_url(url: &str) -> (&str, &str, &str, String) {
    
}

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
    res += &format!("{} /{} {}\r\n", method, path, protocol);
    res += &format!("Host: {}\r\n", host);
    res += "Accept: */*\r\n";
    res += "Connection: close\r\n";
    
    if method == "POST" || method == "PUT" {
        if headers.len() > 0 {
            for head in headers {
                res += head;
            }
            res += "\r\n";
        } else {
            res += "Content-Type: application/json\r\n"
        }
        if let Some(data_str) = data {
            let data_bytes = data_str.as_bytes();
            res += &format!("Content-Length: {}\r\n\r\n", data_bytes.len());
            res += data_str;
            res += "\r\n";
        }
    }

    res += "\r\n";
    res
}

fn parse_resp(resp: &str) -> (&str, &str) {
    let (response_header, response_data) = resp.split_once("\r\n\r\n").unwrap();
    (response_header, response_data)
}


fn main() {
    let matches = get_arguments();

    //argument matching
    let verbose_enabled = matches.contains_id("verbose") && matches.get_flag("verbose"),
    let url = matches.get_one::<String("url").unwrap();
    let data = matches.get_one:<String("data");
    let method = matches.get_one::<String>("x-method");
    let headers: Vec<&str> = matches
        .get_many::<String>("headers")
        .unwrap_or_default()
        .map(|s| s.as_str())
        .collect();

    
}
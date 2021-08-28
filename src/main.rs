use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod request_data;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

//this will get template.html and append the divs with data to body
fn generate_template() -> String {
    let json_csv = request_data::get_data().unwrap(); // -> request data from database request_data::get_data()
    let json_csv: Vec<&str> = json_csv.split(",").collect(); //-> push all the rows to vector json_csv

    let contents = fs::read_to_string("template.html").unwrap();

    let mut html_divs = String::new();
    //loop through the json_csv vector and append it to html_divs
    for e in json_csv.iter() {
        parse_json(&e);
        html_divs += format!("<div class='grid-item'>{}</div>", parse_json(&e)).as_str();
    }

    let formatted_content = contents.replace("{BODY}", &html_divs);
    formatted_content
}

fn parse_json(e: &str) -> String {
    let parsed = json::parse(&e).unwrap();
    parsed["stuff"].to_string()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        generate_template().len(),
        generate_template()
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

use std::{net::TcpStream, io::{Write, LineWriter, stdin, stdout, BufReader, BufRead}, thread, process};

use chat_client::listener::get_tcp_socket;
use chat_client::terminal::Terminal;

/// automatically appends a newline to end and sends it
fn send_line<W: Write>(writer: &mut LineWriter<W>, msg: &str) -> std::io::Result<()> {
    writer.write_all(format!("{msg}\n").as_bytes())
}

/// loops on empty input
fn get_username(msg: &str) -> String {
    print!("{msg}");
    stdout().flush().unwrap();

    let chars = msg.len();

    let mut input = String::new();
    
    stdin().read_line(&mut input).unwrap();

    while input.trim().is_empty() {
        Terminal::move_cursor_up(1);
        Terminal::move_cursor_right(chars);
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
    }

    input.trim().to_string()
}

fn main() {
    print!("Enter the server address (ip), press enter for default: ");
    stdout().flush().unwrap();

    let mut addr = String::new();
    stdin().lock().read_line(&mut addr).unwrap();

    let addr = if !addr.trim().is_empty() {
        addr.trim()
    } else {
        "localhost:3030"
    };

    println!("Connecting to {}...", addr);
    let mut stream = match TcpStream::connect(addr) {
        Ok(stream) => stream,
        Err(_) => {
            eprintln!("No server found!");
            process::exit(-1);
        }
    };
    println!("Successfully connected on port {}!", stream.local_addr().unwrap().port());

    let username = get_username("username: ");

    stream.shutdown(std::net::Shutdown::Read).unwrap();

    let listener = get_tcp_socket().unwrap();

    let listener_address = listener.local_addr().unwrap();

    let mut writer = LineWriter::new(&mut stream);

    send_line(&mut writer, &listener_address.to_string()).unwrap();
    send_line(&mut writer, &username).unwrap();

    drop(writer);

    Terminal::move_cursor_down(26);
    Terminal::erase_screen();
    print!("> ");
    stdout().flush().unwrap();

    thread::spawn(move || {
        let mut writer = LineWriter::new(&mut stream);

        for line in stdin().lines() {
            let line = line.unwrap();

            if line.is_empty() {
                Terminal::move_cursor_up(1);
                Terminal::move_cursor_right(2);
                stdout().flush().unwrap();
                continue;
            }

            send_line(&mut writer, &line).unwrap();

            Terminal::move_cursor_beggining_up(1);
            Terminal::erase_to_end_of_line();
        }
    });

    for connection in listener.incoming() {
        let stream = connection.unwrap();

        let reader = BufReader::new(&stream);

        for line in reader.lines() {
            let line = line.unwrap();

            Terminal::move_cursor_to_column(0);

            print!("{}\n", line);
            print!("> ");

            stdout().flush().unwrap();
        }
    }
}
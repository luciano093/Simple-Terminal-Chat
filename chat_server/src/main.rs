use std::io::{BufReader, BufRead, Read, stdout, stdin, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;

use chat_server::client::{Client, AtomicClients};
use chat_server::server::Server;

fn main() {
    print!("Enter the server port to connect to, press enter for default: ");
    stdout().flush().unwrap();

    let mut port = String::new();
    stdin().lock().read_line(&mut port).unwrap();

    let port: u16 = if !port.trim().is_empty() {
        port.trim().parse().unwrap()
    } else {
        3030
    };

    let server = Server::new().run(format!("127.0.0.1:{}", port));

    println!("Server started on port {}!", server.port());

    let connected_clients = Arc::new(AtomicClients::new());

    for connection in server.incoming() {
        let stream = connection.unwrap();
        let connected_clients = Arc::clone(&connected_clients);

        thread::spawn(|| {
            handle_connection(stream, connected_clients);
        });

        println!("test");
    }
}

fn read_to_string_trimmed<R: Read>(reader: &mut BufReader<R>) -> String {
    let mut string = String::new();

    // Returns an error when the client closes the connection
    drop(reader.read_line(&mut string));

    string.trim().to_string()
}

fn handle_connection(stream: TcpStream, connected_clients: Arc<AtomicClients>) {
    let client_stream_address = stream.peer_addr().unwrap();

    println!("New connection from <{}>", client_stream_address);

    let mut reader = BufReader::new(&stream);

    let client_listener_address = read_to_string_trimmed(&mut reader);
    let username = read_to_string_trimmed(&mut reader);

    connected_clients.send_message(&format!("{} has joined!", username)).unwrap();

    if let Ok(client) = Client::new(client_listener_address, &username) {
        connected_clients.add_client(client_stream_address, client);
    }

    'incoming: for line in reader.lines() {
        let message = match line {
            Ok(msg) => msg,
            Err(_) => {
                break;
            }
        };

        let mut connected_clients = connected_clients.lock().unwrap();
        for (_, client) in connected_clients.iter_mut() {
            let formatted_message = format!("{}: {}", username, message);

            // the client is disconnected when the message can't reach it 
            if let Err(_) = client.send_message(&formatted_message) {
                break 'incoming;
            }
        }
    }

    println!("<{}> had disconnected", client_stream_address);
    connected_clients.remove_client(client_stream_address);
    connected_clients.send_message(&format!("{} has disconnected!", username)).unwrap();
}
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::f64::consts::PI;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from client");

    if bytes_read == 0 {
        return; // Connexion fermée par le client
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut parts = request.split_whitespace();

    let function = match parts.next() {
        Some(f) => f.to_lowercase(), // Normalisation en minuscule
        None => {
            stream.write(b"Invalid request").unwrap();
            return;
        }
    };

    let param1: f64 = match parts.next().and_then(|p| p.parse().ok()) {
        Some(p) if p >= 0.0 => p, // Vérification des nombres négatifs
        _ => {
            stream.write(b"Invalid first parameter (must be non-negative)").unwrap();
            return;
        }
    };

    let param2: Option<f64> = parts.next().and_then(|p| p.parse().ok()).filter(|&p| p >= 0.0);
    let param3: Option<f64> = parts.next().and_then(|p| p.parse().ok()).filter(|&p| p >= 0.0);

    let response = match function.as_str() {
        "sr" => match param2 {
            Some(p2) => sr(param1, p2),
            None => "Missing or invalid second parameter".to_string(),
        },
        "st" => match param2 {
            Some(p2) => st(param1, p2),
            None => "Missing or invalid second parameter".to_string(),
        },
        "sc" => sc(param1),
        "ss" => ss(param1),
        "vp" => match (param2, param3) {
            (Some(p2), Some(p3)) => vp(param1, p2, p3),
            _ => "Missing or invalid second or third parameter".to_string(),
        },
        "vs" => vs(param1),
        _ => "Invalid function".to_string(),
    };

    stream.write(response.as_bytes()).expect("Failed to write to client");
    stream.flush().expect("Failed to flush stream");
}

fn sr(length: f64, width: f64) -> String {
    format!("{}", length * width)
}

fn st(base: f64, height: f64) -> String {
    format!("{}", 0.5 * base * height)
}

fn sc(radius: f64) -> String {
    format!("{}", PI * radius.powi(2))
}

fn ss(radius: f64) -> String {
    format!("{}", 4.0 * PI * radius.powi(2))
}

fn vp(length: f64, width: f64, height: f64) -> String {
    format!("{}", length * width * height)
}

fn vs(radius: f64) -> String {
    format!("{}", (4.0 / 3.0) * PI * radius.powi(3))
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6000").expect("Failed to bind port");
    println!("Server listening on port 6000...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::f64::consts::PI;

// Fonction pour gérer un client connecté
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("Échec de la lecture du client");

    if bytes_read == 0 {
        return; // Connexion fermée par le client
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut parts = request.split_whitespace();

    // Extraction du nom de la fonction
    let function = match parts.next() {
        Some(f) => f.to_lowercase(), // Normalisation en minuscule
        None => {
            stream.write(b"Requête invalide").unwrap();
            return;
        }
    };

    // Extraction et validation des paramètres
    let param1: f64 = match parts.next().and_then(|p| p.parse().ok()) {
        Some(p) if p >= 0.0 => p, // Vérification des nombres négatifs
        _ => {
            stream.write(b"Premier paramètre invalide (doit être non négatif)").unwrap();
            return;
        }
    };

    let param2: Option<f64> = parts.next().and_then(|p| p.parse().ok()).filter(|&p| p >= 0.0);
    let param3: Option<f64> = parts.next().and_then(|p| p.parse().ok()).filter(|&p| p >= 0.0);

    // Exécution de la fonction demandée
    let response = match function.as_str() {
        "sr" => match param2 {
            Some(p2) => sr(param1, p2),
            None => "Deuxième paramètre manquant ou invalide".to_string(),
        },
        "st" => match param2 {
            Some(p2) => st(param1, p2),
            None => "Deuxième paramètre manquant ou invalide".to_string(),
        },
        "sc" => sc(param1),
        "ss" => ss(param1),
        "vp" => match (param2, param3) {
            (Some(p2), Some(p3)) => vp(param1, p2, p3),
            _ => "Deuxième ou troisième paramètre manquant ou invalide".to_string(),
        },
        "vs" => vs(param1),
        _ => "Fonction invalide".to_string(),
    };

    // Envoi de la réponse au client
    stream.write(response.as_bytes()).expect("Échec de l'écriture vers le client");
    stream.flush().expect("Échec de la transmission des données");
}

// Calcul de la surface d'un rectangle
fn sr(length: f64, width: f64) -> String {
    format!("{}", length * width)
}

// Calcul de la surface d'un triangle
fn st(base: f64, height: f64) -> String {
    format!("{}", 0.5 * base * height)
}

// Calcul de la surface d'un cercle
fn sc(radius: f64) -> String {
    format!("{}", PI * radius.powi(2))
}

// Calcul de la surface d'une sphère
fn ss(radius: f64) -> String {
    format!("{}", 4.0 * PI * radius.powi(2))
}

// Calcul du volume d'un prisme rectangulaire
fn vp(length: f64, width: f64, height: f64) -> String {
    format!("{}", length * width * height)
}

// Calcul du volume d'une sphère
fn vs(radius: f64) -> String {
    format!("{}", (4.0 / 3.0) * PI * radius.powi(3))
}

fn main() {
    // Création du serveur TCP
    let listener = TcpListener::bind("127.0.0.1:6000").expect("Échec de la liaison au port");
    println!("Serveur à l'écoute sur le port 6000...");

    // Acceptation des connexions entrantes
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Échec de la connexion : {}", e);
            }
        }
    }
}

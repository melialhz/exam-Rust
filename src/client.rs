use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    // Tentative de connexion au serveur
    let mut stream = match TcpStream::connect("127.0.0.1:6000") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Impossible de se connecter au serveur");
            return;
        }
    };

    // Récupération du nom de la fonction à exécuter
    let function = get_function_name();
    // Récupération du premier paramètre (valeur positive)
    let param1 = get_positive_number("Entrez le premier paramètre :");

    // Construction de la requête avec la fonction et le premier paramètre
    let mut request = format!("{} {}", function, param1);

    // Vérification si une deuxième valeur est nécessaire
    if ["sr", "st", "vp"].contains(&function.as_str()) {
        let param2 = get_positive_number("Entrez le deuxième paramètre :");
        request = format!("{} {}", request, param2);
    }

    // Vérification si une troisième valeur est nécessaire
    if function == "vp" {
        let param3 = get_positive_number("Entrez le troisième paramètre :");
        request = format!("{} {}", request, param3);
    }

    // Envoi de la requête au serveur
    stream.write(request.as_bytes()).expect("Échec de l'écriture vers le serveur");
    stream.flush().expect("Échec de la transmission des données");

    // Lecture de la réponse du serveur
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("Échec de la lecture depuis le serveur");
    if bytes_read == 0 {
        println!("Le serveur a fermé la connexion.");
        return;
    }
    
    // Affichage de la réponse du serveur
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Résultat : {}", response.trim());
}

// Fonction pour récupérer le nom d'une fonction valide auprès de l'utilisateur
fn get_function_name() -> String {
    loop {
        let mut input = String::new();
        println!("Entrez le nom de la fonction (sr, st, sc, ss, vp, vs) :");
        io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");
        let function = input.trim().to_lowercase(); // Conversion en minuscule

        // Vérification si la fonction est valide
        if ["sr", "st", "sc", "ss", "vp", "vs"].contains(&function.as_str()) {
            return function;
        }

        println!("Nom de fonction invalide. Veuillez entrer un nom valide.");
    }
}

// Fonction pour récupérer un nombre positif auprès de l'utilisateur
fn get_positive_number(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Échec de la lecture de l'entrée");
        
        // Vérification si l'entrée est un nombre positif
        match input.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return input.trim().to_string(),
            _ => println!("Entrée invalide. Veuillez entrer un nombre positif."),
        }
    }
}

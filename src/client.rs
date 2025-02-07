use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:6000") {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Could not connect to server");
            return;
        }
    };

    let function = get_function_name();
    let param1 = get_positive_number("Enter the first parameter:");

    let mut request = format!("{} {}", function, param1);

    if ["sr", "st", "vp"].contains(&function.as_str()) {
        let param2 = get_positive_number("Enter the second parameter:");
        request = format!("{} {}", request, param2);
    }

    if function == "vp" {
        let param3 = get_positive_number("Enter the third parameter:");
        request = format!("{} {}", request, param3);
    }

    stream.write(request.as_bytes()).expect("Failed to write to server");
    stream.flush().expect("Failed to flush stream");

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");
    if bytes_read == 0 {
        println!("Server closed the connection.");
        return;
    }
    
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Result: {}", response.trim());
}

fn get_function_name() -> String {
    loop {
        let mut input = String::new();
        println!("Enter the function name (sr, st, sc, ss, vp, vs):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let function = input.trim().to_lowercase(); // Convertir en minuscule

        if ["sr", "st", "sc", "ss", "vp", "vs"].contains(&function.as_str()) {
            return function;
        }

        println!("Invalid function name. Please enter a valid one.");
    }
}

fn get_positive_number(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => return input.trim().to_string(),
            _ => println!("Invalid input. Please enter a non-negative number."),
        }
    }
}

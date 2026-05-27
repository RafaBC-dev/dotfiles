use std::env;
use std::fs;
use std::process;
use colored::*;

fn main() {
    // Los argumentos los metemos en un vector para tener un mejor orden y control sobre ellos
    let args: Vec<String> = env::args().collect();

    // Comprobamos que tienen los argumentos necesarios, sino, cerramos el programa con un mensaje en rojo.
    if args.len() < 3 {
        eprintln!("Error, sintaxis incorrecta. Ejemplo: cargo run Rust poema.txt");
        process::exit(1);
    }

    let searched_word: &String = &args[1]; // 1º argumento (despues de cargo run -> args[0])
    let path: &String = &args[2];          // 2º argumento

    // Manejamos errores de abrir y leer el archivo con un match
    let content: String = match fs::read_to_string(&path) {
        Ok(texto) => texto,
        Err(_) => {
            eprintln!("Error: No se pudo leer el archivo. ¿Seguro que existe y está bien escrito?");
            process::exit(1);
        }
    };

    // Pasamos a minusculas la palabra que queremos buscar para evitar fallos con mayusc(Rust != rust)  
    let searched_word_lower = searched_word.to_lowercase();

    let mut flag: bool = false;

    // Pasamos linea por linea (enumerandolas), si la linea pasada a minusculas 
    // contiene la palabra que buscabamos, la imprimimos completa (la original)
    for (n, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&searched_word_lower) {
            println!("Linea {}: {}", n + 1, line.green());
            flag = true;
        }
    }

    if flag != true {
        eprintln!("No se ha econtrado la palabra {} en {}", searched_word.red(), path);
    }
}
 
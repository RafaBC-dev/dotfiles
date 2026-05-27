/*
 * Escribe una función que reciba dos palabras (String) y retorne
 * verdadero o falso (Bool) según sean o no anagramas.
 * - Un Anagrama consiste en formar una palabra reordenando TODAS
 *   las letras de otra palabra inicial.
 * - NO hace falta comprobar que ambas palabras existan.
 * - Dos palabras exactamente iguales no son anagrama.
 */

// 1º Crear funcion para leer palabras (input)
// 2º Crear funcion para ordenar ambas palabras y comprobar si son anagramas
// como no pueden ser iguales la comprobacion tendra tres pasos, 1º word1 == word2,
// 2º word1.sort().to_lowercase() == word2.sort().to_lowercase() y si no pues no
// son anagramas

use std::io;

fn is_anagram(word1: &str, word2: &str) -> bool {
    let w1 = word1.to_lowercase();
    let w2 = word2.to_lowercase();

    if w1 == w2 {
        return false;
    }

    let mut chars1: Vec<char> = word1.chars().collect();
    let mut chars2: Vec<char> = word2.chars().collect();

    chars1.sort();
    chars2.sort();

    chars1 == chars2
}

fn main() {
    println!("Introduce la primera palabra:");
    let mut word1 = String::new();
    io::stdin().read_line(&mut word1).expect("Error");

    println!("Introduce la segunda palabra");
    let mut word2 = String::new();
    io::stdin().read_line(&mut word2).expect("Error");

    let clean_word1 = word1.trim();
    let clean_word2 = word2.trim();

    if is_anagram(clean_word1, clean_word2) {
        println!("SI son anagramas {} y {}", word1, word2);
    } else {
        println!("NO son anagramas {} y {}", word1, word2);
    }
}

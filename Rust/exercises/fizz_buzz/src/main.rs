// Escribe un programa que muestre por consola (con un print) los numeros del 1 al 100
// (ambos incluidos y con un salto de linea entre cada impresión), sustituyendo los siguientes:
// - Multiplos de 3 por la palabra "fizz",
// - Multiplos de 5 por la palabra "buzz".
// - Multiplos de 3 y 5 a la vez por la palabra "fizzbuzz".

fn main() {
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{i}"),
        }
    }
}

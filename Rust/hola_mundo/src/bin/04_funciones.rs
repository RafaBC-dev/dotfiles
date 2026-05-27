#![allow(unused_variables)]


// Sintaxis: fn nombre(parametro: Tipo) -> TipoRetorno { ... }
fn sumar(a: i32, b: i32) -> i32 {
    a + b  // Rust entiende que es lo que debe devolver. No hace falta ni return ni ";"
}

fn destruir_texto(texto: String) {
    println!("Me han regalado este texto: {}", texto);
    // Como no he usado "&", esta función se convierte en la DUEÑA del texto.
    // Cuando la función termina aquí, la variable se destruye en la memoria.
}

fn leer_texto(texto: &String) {
    println!("Solo estoy mirando este texto: {}", texto);
    // Usamos "&". La función mira, pero no es la dueña.
    // Cuando termina, devuelve el control a quien la llamó.
}

fn modificar_texto(texto: &mut String) {
    texto.push_str(" ahora modificado.");
    // Usamos "&mut". Podemos alterar el valor original directamente.
}


fn main() {
    // 1. Uso básico
    let num1: i32 = 234;
    let num2: i32 = 32;
    let resultado: i32 = sumar(num1, num2); // Las variables se convieren en los parametros
    let resultado2: i32 = sumar(234, 234);  // Pasarle los valores directamente tambien es valido
    println!("La suma es: {}", resultado);
    println!("La suma es: {}", resultado2);

    // 2. Jugando con los préstamos (Borrowing)
    let mut mi_mensaje: String = String::from("Hola Rust"); // Creo un String mutable (let mut)

    // PRESTAMOS para que lo lea (usamos &)
    leer_texto(&mi_mensaje); 
    println!("Después de leer: {}", mi_mensaje); // Funciona, aún lo tenemos

    // PRESTAMOS para que lo modifique (usamos &mut)
    modificar_texto(&mut mi_mensaje);
    println!("Después de modificar: {}", mi_mensaje); // Funciona, y está cambiado

    // REGALAMOS la variable (sin &)
    destruir_texto(mi_mensaje);
    
    // Esto fallaria porque la variable ya no existe, se la pasamos antes a la funcion, por lo que se adueñó de ella y la destruyó al terminar
    // println!("¿Sigue vivo? {}", mi_mensaje); -> ERROR: "value borrowed here after move"
}
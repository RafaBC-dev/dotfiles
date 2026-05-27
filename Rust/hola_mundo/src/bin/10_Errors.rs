#![allow(unused_variables)]
#![allow(dead_code)]

// En Rust, las funciones que pueden fallar no devuelven el dato directamente.
// Devuelven un Result<TipoExito, TipoError>.

fn dividir(numerador: f64, denominador: f64) -> Result<f64, String> {
    if denominador == 0.0 {
        // devolvemos la variante Err con el mensaje
        Err(String::from("Error: division por cero."))
    } else {
        // con la variante Ok devolvemos el resultado 
        Ok(numerador/denominador)
    }
}

// Tambien podemos manejar errores con el operador '?' 
// Sirve para encadenar operaciones que puede fallar sin hacer "match" gigantes. Donde se use '?' tambien debe devolver un Result

fn operacion_compleja (a: f64, b: f64, c: f64) -> Result<f64, String> {
    // Si 'dividir' va bien, saca el número del 'Ok()' y lo guarda en paso1.
    // Si 'dividir' falla, hace un 'return Err()' automático y cancela esta función.
    let paso1 = dividir(a, b)?;
    let paso2 = dividir(paso1, c)?;

    Ok(paso2) 
}

fn main () {
    // --- ERRORES IRRECUPERABLES (Panic) ---
    // Si sabes que tu programa no puede continuar de ninguna manera:
    // panic!("Fuego en el servidor. Apagando todo."); 
    // (Descomenta la línea de arriba para ver cómo explota el programa)

    // --- Manejo Seguro (Match) ---
    println!("--- Intentando divir 10 entre 2 ---");
    let intento_bueno = dividir(10.0, 2.0);

    match intento_bueno {
        Ok (resultado) => println!("Exito: {}", resultado),
        Err(mensaje) => println!("Error: {}", mensaje),
    }

    println!("\n--- Intentando dividir 10 entre 0 ---");
    let intento_malo = dividir(10.0, 0.0);

    match intento_malo {
        Ok(resultado) => println!("Exito: {resultado:?}"),
        Err(error) => println!("Exito: {error:?}"),
    }

    // --- Atajos (Peligrosos pero muy usados)
    // .unwrap() -> "Confio en que esto es Ok. Si es Err, haz PANIC."
    let seguro_que_funciona = dividir(20.5, 5.0).unwrap();
    println!("Resultado: {}", seguro_que_funciona);

    // .expect() -> Igual que unwrap, pero te deja poner un mensaje de error personalizado antes de explotar.
    // let me_la_juego = dividir(10.0, 0.0).expect("Creí que el denominador no era cero");

    
    match operacion_compleja(100.0, 2.0, 3.938) {
        Ok(resultado) => println!("{:.3}", resultado),
        Err(error) => println!("{}", error),
    }
}
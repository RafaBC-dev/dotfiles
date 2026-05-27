// Hay que importar los diccionarios de la libreria estandar ,ya que estos no vienen  por defecto
use std::collections::HashMap;

fn main() {
    // Clave: String(nombre jugador) -> Valor: i32 (puntuacion)
    let mut puntuaciones: HashMap<String, i32> = HashMap::new(); // Creamos el diccionario

    // Insertar datos
    puntuaciones.insert(String::from("Marisa"), 100);
    puntuaciones.insert(String::from("Rafae"), 69);
    puntuaciones.insert(String::from("Rafae"), 150); // He SOBREESCRITO el valor 69, ahora vale 150

    // Leer los datos (Acceso seguro)
    let jugador: String = String::from("Marisa");

    // .get() pide una referencia a la clave (&jugador) y devuelve un Option
    match puntuaciones.get(&jugador) {
        Some(puntos) => println!("{} tiene {} puntos", jugador, puntos),
        None => println!("El jugador no existe en la base de datos."),
    }

    // Comprueba si el jugador existe y si no, lo crea
    puntuaciones.entry(String::from("Pepelu")).or_insert(50);

    let jugador_pepelu: String = String::from("Pepelu");

    match puntuaciones.get(&jugador_pepelu) {
        Some(puntos) => println!("{} tiene {} puntos.", jugador_pepelu, puntos),
        None => println!("El jugador no existe en la DB."),
    }

    // Como Rafae ya existe, esta línea NO hace nada
    puntuaciones.entry(String::from("Rafae")).or_insert(0);

    // Actualizar un valor basado en el anterior (Ej: sumar puntos)
    let puntos_pepelu = puntuaciones.entry(String::from("Pepelu")).or_insert(98);
    *puntos_pepelu += 456;
    
    // Otra forma de mirar los puntos actuales es usando directamente esa referencia creada con un puntero
    println!("Pepelu tiene {} puntos actually (actualmente).", *puntos_pepelu);

    match puntuaciones.get(&jugador_pepelu) {
        Some(puntos) => println!("{} tiene {} puntos.", jugador_pepelu, puntos),
        None => println!("El jugador no existe en la DB."),
    }

    // Iterar el mapa
    println!("\n--- Tabla de Clasificacion ---");
    for (v, k) in &puntuaciones {
        println!("{}: {}", v, k);
    }

}
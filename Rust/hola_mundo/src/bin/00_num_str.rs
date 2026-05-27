#![allow(unused_variables)]
#![allow(unused_mut)]

fn main () {

    // --- Numeros ---
    // Enteros 
    let x:i32 = -35;                // immutable variable 32 bits
    let mut y: i64 = 59386453457;   // mutable variable 64 bits

    // Sin signo 
    let unsigned_num: u32 = 235;        

    // Float (decimales)
    let pi: f64 = 3.14259;
    let negativo_decimal: f32 = -5.5;

    let ratio: f64= 10 as f64;  // Casting 
    println!("{ratio:.2}");

    let absoluto: i32 = x.abs();                   // Valor absoluto
    let potencia: u32 = unsigned_num.pow(3);  // Potencia 
    let maximo: i32 = x.max(unsigned_num as i32);  // Compara y devuelve el maximo 
    let clamp: i32 = 500.clamp(0, 100);   // Lo "encierra" en ese rango
    let millones: u64 = 1_000_000_u64;

    // Metodos utiles para num decimales
    let redondeo = pi.round();       // 3.0 (redondea al mas cercano)
    let hacia_abajo = pi.floor();    // 3.0 (redondea hacia abajo)
    let hacia_arriba = pi.ceil();    // 4.0 (redonde hacia arriba)
    let valor_abs = negativo_decimal.abs(); // 5.5

    let pi_real: f64 = std::f64::consts::PI; // Numero pi exacto("") importado directamente
    println!("{pi_real:?}");


    // --- Strings y &str ---
    let nombre: &str = "Juanlu";                       // Esto es un puntero (referencia) pero no se puede modificar
    let apellido: String = String::from("Socarrá");    // Esto es un string, se puede modificar y asignar a otra variable
    let mut saludo: String = String::from("Hola");

    // Añadir texto (Mutacion)
    saludo.push_str(" y adios, ");    // añade un &str (comilla dobles), ahora saludo = "Hola y adios, "
    println!("{}", saludo);                   
    saludo.push_str(nombre);        // le pasamos la variable "Juanlu", ahora saludo = "Hola y adios, Juanlu"
    saludo.push('!');                   // push() añade un solo caracter (comillas simples '')

    // Metodos de consulta
    let longitud: usize = saludo.len();    // cuenta BYTES, no letras (si que devuelve numero de elementos en un vector, por ej)
    println!("{longitud:?}");
    let esta_vacio: bool = saludo.is_empty();          // false (es "lo mismo" que if saludo.len() == 0 {}, pero mas legible y rapido)
    let contiene_juan: bool = saludo.contains("Juan"); // true (antes hicimos push_str(nombre))
    println!("{}", contiene_juan);
    println!("{}", saludo);

    // Transformaciones (Devuelven un String nuevo, no modifican el original)
    let mayusc: String = saludo.to_uppercase();
    let reemplazo: String = saludo.replace("Juanlu", "Rustacen");
    let reemplazo2: String = saludo.replace(&saludo, "Rustacen");
    let no_spaces: &str = "    texto    con espacios  .".trim();  // solo quita los espacios al principio y al final (no entre medio)

    println!("{reemplazo:?}");
    println!("{}", saludo);
    println!("{reemplazo2:?}");
    println!("{}", saludo);
    println!("{no_spaces:?}");

    // Slicing: cortar un String (bytes, no puedo cortar un emoji por la mitad, por ejemplo)
    let primeros_cuatro = &saludo[0..10];
    println!("{primeros_cuatro:?}");
    
    // !format()
    let formateado: String = format!("{} {}. Longitud del saludo: {}", nombre, apellido, longitud);
    println!("{formateado:?}");
    
    println!("Saludo final: {saludo}");
    println!("Tarjeta: {formateado}");    
}
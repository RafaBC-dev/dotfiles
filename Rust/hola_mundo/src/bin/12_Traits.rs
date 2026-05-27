#![allow(unused_variables)]
#![allow(dead_code)]

// ==========================================
// 1. DEFINIR EL TRAIT (El "Contrato")
// ==========================================
trait Describible {
    // Quien firme este contrato, DEBE crear una función con esta firma exacta:
    fn describir(&self) -> String;
    
    // ¡Opcional! Los traits pueden tener un comportamiento "por defecto".
    // Si la estructura no escribe el suyo propio, usará este.
    fn imprimir_consola(&self) {
        println!("INFO: {}", self.describir());
    }
}

// ==========================================
// 2. NUESTRAS ESTRUCTURAS DE DATOS
// ==========================================
struct Usuario {
    nombre: String,
    edad: u8,
}

struct Producto {
    nombre: String,
    precio: f64,
}

// ==========================================
// 3. FIRMAR EL CONTRATO (Implementar el Trait)
// ==========================================
// Aquí le damos el "superpoder" del Trait a nuestras structs

impl Describible for Usuario {
    fn describir(&self) -> String {
        format!("Usuario {} ({} años)", self.nombre, self.edad)
    }
}

impl Describible for Producto {
    fn describir(&self) -> String {
        format!("Producto '{}' - ${}", self.nombre, self.precio)
    }
}

// ==========================================
// 4. LA MAGIA: Funciones que aceptan Traits
// ==========================================

// OPOCIÓN A: Sintaxis 'impl Trait' (La más limpia y moderna)
// Le decimos: "Acepto CUALQUIER COSA, siempre que haya firmado el contrato Describible"
fn notificar(item: &impl Describible) {
    println!("Enviando notificación... {}", item.describir());
}

// OPCIÓN B: Los Genéricos con "Límites" (Trait Bounds)
// Es lo mismo de arriba, pero usando la <T>. Es útil si tienes muchos parámetros.
fn notificar_generico<T: Describible>(item1: &T, item2: &T) {
    // Como sabemos que T implementa Describible, podemos usar sus métodos con total seguridad.
    item1.imprimir_consola(); 
    item2.imprimir_consola();
}


fn main() {
    let juan = Usuario {
        nombre: String::from("Juanlu"),
        edad: 30,
    };

    let portatil = Producto {
        nombre: String::from("MacBook"),
        precio: 1200.50,
    };

    // ¡Mira esto! La misma función 'notificar' acepta un Usuario y un Producto.
    // Esto en Rust es magia pura, porque son tipos totalmente distintos.
    println!("--- Usando impl Trait ---");
    notificar(&juan);
    notificar(&portatil);

    println!("\n--- Usando métodos por defecto del Trait ---");
    // Llamamos al método que el Trait nos regaló "gratis"
    juan.imprimir_consola();
    portatil.imprimir_consola();
}´
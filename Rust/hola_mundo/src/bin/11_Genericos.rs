#![allow(unused_variables)]
#![allow(dead_code)]

// ==========================================
// 1. GENÉRICOS EN FUNCIONES
// ==========================================
// La letra 'T' significa "Type" (Tipo). Puedes usar cualquier letra, pero T es la convención.
// Le decimos a la función: "Vas a recibir un tipo <T>. El parámetro 'dato' es de ese tipo T".

fn devolver_lo_mismo<T>(dato: T) -> T {
    // Esto realmente solo devuelve el dato, pero se puede comprobar que acepta cualquier tipo de dato
    dato
}

// Para dos tipo distintos pues se usan dos letras
fn agrupar_en_tupla<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

// ==========================================
// 2. GENÉRICOS EN ESTRUCTURAS (STRUCTS)
// ==========================================
// Imagina que haces un juego y necesitas coordenadas. 
// A veces las quieres exactas (f64) y a veces en píxeles (i32).

#[derive(Debug)]
struct Punto<T> {
    x: T,
    y: T, // x e y tienen que ser obligatoriamente del MISMO tipo 'T'
}

#[derive(Debug)]
struct PuntoMixto<T, U> {
    x: T,
    y: U, // Aquí x e y pueden ser de tipos distintos
}

// ==========================================
// 3. GENÉRICOS EN MÉTODOS (Bloque impl)
// ==========================================
// Tienes que poner el <T> después del impl para que Rust sepa que estás implementando algo genérico.

impl<T> Punto<T> {
    // Devuelve una referencia al valor X
    fn obtener_x(&self) -> &T {
        &self.x
    }
}

// Se puede hacer métodos que SOLO existan para un tipo específico
// Este método solo existirá si creas un Punto con números f32.
impl Punto<f32> {
    fn distancia_al_origen(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main () {
    println!("Funciones Genericas...");
    let numero = devolver_lo_mismo(5);       // Rust deduce que T es i32
    let texto = devolver_lo_mismo("Hola");  // Rust deduce que U es &str

    let tupla = agrupar_en_tupla(100, "Puntos"); // T es i32, U es &str
    println!("{tupla:#?}");

    println!("Structs Genericos...");
    let p_entero: Punto<i32> = Punto {x: 5, y: 10};
    println!("{p_entero:#?}");

    let p_decimal: Punto<f32> = Punto { x: 1.5, y: 5.234 };
    println!("{p_decimal:#?}");

    // ERROR: 
    //let p_error = Punto { x: 5, y: 4.0 }; // x (i32) e y (f64) no son el mismo tipo 'T'

    let p_mezclado = PuntoMixto {x: 5, y: "Puntos"};
    println!("{p_mezclado:#?}");
    
    println!("Punto entero: {:?}", p_entero);
    println!("Valor X del punto decimal: {}", p_decimal.obtener_x());
    println!("Distancia: {:.2}", p_decimal.distancia_al_origen()); // Solo funciona porque T es f32

}
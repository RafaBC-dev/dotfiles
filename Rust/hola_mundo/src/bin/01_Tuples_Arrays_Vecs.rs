#![allow(unused_variables)]
#![allow(unused_mut)]

/* Recomendaciones generales de uso:
-----------------------------------
Tipo datos  |  Tamaño   | Eleccion
-----------------------------------  
Distintos   | Fijo      | Tuplas   | => Si tiene un tamaño fijo pero distinto tipo de datos
Iguales     | Fijo      | Arrays   | => Si tienen tanto el mismo tipo de datos como un tamaño fijo (mas rapido, memoria Stack)
Iguales     | Variable  | Vectores | => Si tiene mismo tipo pero tamaño variable (puede crecer, memoria Heap)
*/

fn main() {
// --- Tuplas ---

    // nombre (tipo1, tipo2) = (valor1, valor2)
    let mut mi_tupla: (i32, &str) = (35, "Juanlu");  

    let edad = mi_tupla.0;     // Acceso por punto (indice)
    mi_tupla.1 = "Socarrá";         // Modificiacion (si es mut)
    
    let (años, nombre) = mi_tupla; // Sacar los valores en variables 


// --- Arrays --- 

    // nombre: [tipo; tamaño] = [valores] => Declaramos todo; tipo fijo y tamaña fijo
    let mut mi_array: [i32; 3] = [10, 20, 30];

    // Metodos
    let longitud = mi_array.len();
    let primero = mi_array[0];
    let existe = mi_array.contains(&30);   

    let ceros: [i32; 50] = [0; 50];

    println!("{ceros:?}");
    println!("{mi_tupla:?}");
    println!("{longitud:?}");
    println!("{existe:?}");
    println!("{mi_array:?}");
    println!("{primero:?}");

// --- Vectores ---

    let mut v: Vec<i32> = vec![1, 2, 3];  // Creacion "rapida"

    v.push(4);                      // Añade 4 al final
    v.pop();                        // Quita el ultimo valor
    v.insert(1, 99); // Inserta un valor en el vector por indice(posicion, valor)
    v.remove(0);             // Quita el elemento del indice indicado
    v.sort();                      // Ordena de menor a mayor
    v.clear();                     // Vacia el vector
    
    let vacio = v.is_empty(); // true (o false)

    let mut num_vec = vec![10, 20, 30, 40, 55];

    // .get() devuelve un Option (Some o None), nunca hace Panic. Esto hace que sea seguro, una especie de "try/except"
    match num_vec.get(100) {
        Some(numero) => println!("El numero es: {}", numero),   // estamos buscando el indice 100
        None => println!("Ese indice no existe."), // Se ejecuta en caso de que Some diera error (como es este caso, ya que mi vector tiene tamaño 5)
    }
}  
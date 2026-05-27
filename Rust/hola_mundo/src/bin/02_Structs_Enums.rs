// #! (Inner Attribute): "Afecta a todo este archivo"
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


//Estructuras    
#[derive(Debug)]    // # (Outer Attribute): solo afecta a lo que esta justo debajo -> Usuario
struct Usuario {
    dni: i32,
    email: String,
    activo: bool,
}

impl Usuario {
    // 1. Método "Constructor" (Por convención se suele llamar 'new')
    // No usa 'self' porque se ejecuta ANTES de que el usuario exista.
    fn new(dni_nuevo: i32, email_nuevo: &str) -> Usuario {
        Usuario {
            dni: dni_nuevo,
            email: String::from(email_nuevo),
            activo: true, 
        }
    }
    // 2. Método de Lectura (Usa '&self')
    // Igual que pasar '&', solo mira los datos, no los cambia.
    fn imprimir_resumen(&self) {
        println!("Usuario {} - Email: {}", self.dni, self.email);
    }

    // 3. Método de Escritura/Modificación (Usa '&mut self')
    // Necesita '&mut' para poder cambiar el valor de 'activo'.
    fn desactivar_cuenta(&mut self) {
        self.activo = false;
        println!(" La cuenta {} ha sido desactivada.", self.dni);
    }
}


enum Estado {
    Conectado,
    Desconectado,
    Error(u32),     // Enum con datos dentro
}

impl Estado {
    fn describir(&self) {
        match self {
            Estado::Conectado => println!("El sistema funciona correctamente."),
            Estado::Desconectado => println!("El cable esta desenchufao."),
            Estado::Error(error) => println!("Fallo critico. Codigo: {}", error),
        }
    }
}


fn main() {
    
    // Crear usuario
    let mut pepe: Usuario = Usuario::new(123455, "pepe@gmail.com");

    // Usar metodos propios del usuario
    pepe.imprimir_resumen();
    pepe.desactivar_cuenta();

    println!("Estado actual de Pepe's account:\n{pepe:#?}\n");

    // Usar metodos Enum
    let actual = Estado::Error(404);
    actual.describir();
}
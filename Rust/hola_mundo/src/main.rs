#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}


use std::fmt;

struct OtraStruct(i32);

impl fmt::Display for OtraStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Valor: {}", self.0)
    }
}

fn main() {
    let rust: String = String::from("I'm a rustacean!");
    let number: i32 = 1;
    let num_diff_base: i64 = 69420;

    let name = "Peter";
    let age = 32;
    let peter = Person { name, age };

    println!("Hello, world!");
    println!("{rust:?}");
    println!("{}", rust);
    println!("{0}, esto es {1}. {1}, este es {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("Base 10:               {}", num_diff_base); // base
    println!("Base 2 (binario):      {:b}", num_diff_base); // 10000111100101100
    println!("Base 8 (octal):        {:o}", num_diff_base); // 207454
    println!("Base 16 (hexadecimal): {:x}", num_diff_base); // 10f2c
    println!("{number:>5}");

    println!("{:?}", Deep(Structure(7))); //Se imprime tal cual
    println!("{:?}", Structure(4));
    println!("{:#?}", peter); //Imprime la estructura que hemos asinado a peter y con salto de linea
}

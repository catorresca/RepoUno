use std::io;
use rand::RngExt;

fn main() {
    println!("Adivina el numero.");

    let numero_secreto = rand::rng().random_range(1..=100);

    println!("El numero secreto es: {numero_secreto}");

    println!("Por favor ingresa tu adivinanza.");

    let mut adivina = String::new(); //variable mutable

    io::stdin()
        .read_line(&mut adivina)
        .expect("Fallo al leer la linea!!!");

    println!("Usted supuso: {adivina}");
}
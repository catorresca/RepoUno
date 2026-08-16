use std::io;

fn main() {
    println!("Adivina el numero.");

    println!("Por favor ingresa tu adivinanza.");

    let mut adivina = String::new(); //variable mutable

    io::stdin()
        .read_line(&mut adivina)
        .expect("Fallo!!!");

    println!("Usted supuso: {adivina}");
}
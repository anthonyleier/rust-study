// Crie um programa em Rust que leia um número inteiro e exiba a sua tabuada

pub fn tabuada(numero: i32) {
    for i in 0..11{
        println!("{} x {} = {}", numero, i, numero*i);
    }
}

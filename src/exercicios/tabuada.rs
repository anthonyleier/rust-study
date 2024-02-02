// Faça uma função que receba um número inteiro e exiba a sua tabuada

pub fn tabuada(numero: i32) {
    for i in 0..11{
        println!("{} x {} = {}", numero, i, numero*i);
    }
}

fn main(){
    tabuada(10);
}
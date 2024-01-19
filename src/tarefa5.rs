// Crie um programa em Rust que leia uma sequência de números reais e exiba a soma dos números pares

use std::io;

fn converter_string_to_float(texto: &String) -> f64 {
    let numero: f64 = texto.trim().parse::<f64>().unwrap();
    return numero;
}

pub fn soma_pares() {
    let mut entrada = String::new();
    let mut numeros: Vec<f64> = Vec::new();
    let mut soma: f64 = 0.0;

    loop {
        io::stdin().read_line(&mut entrada).expect("Erro");
        entrada = entrada.trim().to_string();

        if entrada == "sair" {
            break;
        }

        let numero_entrada: f64 = converter_string_to_float(&entrada);
        numeros.push(numero_entrada);

        entrada.clear();
    }

    for numero in numeros {
        if numero % 2.0 == 0.0 {
            soma += numero;
        }
    }

    println!("A soma dos pares é {}", soma);
}

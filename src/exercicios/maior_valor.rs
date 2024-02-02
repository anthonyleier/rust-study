// Faça uma função que receba um vetor de números inteiros e retorne o maior número encontrado no vetor

pub fn maior_valor_vetor_vec(vetor: Vec<i32>) -> i32 {
    let mut maior_valor = vetor[0];
    for item in vetor {
        if item > maior_valor {
            maior_valor = item;
        }
    }
    return maior_valor;
}

pub fn maior_valor_vetor_array(vetor: &[i32]) -> i32 {
    let mut maior_valor = vetor[0];
    for item in vetor {
        if *item > maior_valor {
            maior_valor = *item;
        }
    }
    return maior_valor;
}

fn main() {
    let vetor_entrada = vec![423, 567, 12, 3, 25];
    let maior_valor1 = maior_valor_vetor_vec(vetor_entrada.clone());
    println!(
        "Vetor: {:?} -> Maior valor: {}",
        vetor_entrada, maior_valor1
    );

    let array_entrada = [423, 567, 12, 3, 25];
    let maior_valor2 = maior_valor_vetor_array(&array_entrada);
    println!(
        "Array: {:?} -> Maior valor: {}",
        array_entrada, maior_valor2
    );
}

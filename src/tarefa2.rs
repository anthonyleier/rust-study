// Escreva uma função em Rust que receba um vetor de números inteiros e retorne o maior número encontrado no vetor

pub fn maior_valor_vetor_vec(vetor: Vec<i32>) -> i32{
    let mut maior_valor = vetor[0];
    for item in vetor {
        if item > maior_valor {
            maior_valor = item;
        }
    }
    return maior_valor;
}

pub fn maior_valor_vetor_array(vetor: &[i32]) -> i32{
    let mut maior_valor = vetor[0];
    for item in vetor {
        if *item > maior_valor {
            maior_valor = *item;
        }
    }
    return maior_valor;
}

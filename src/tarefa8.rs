// Faça uma função que verifique se uma string é a permutação de outra

use std::collections::HashMap;

pub fn e_permutacao(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }

    let mut contador_caracteres = HashMap::new();
    for caractere in str1.chars() {
        *contador_caracteres.entry(caractere).or_insert(0) += 1;
    }

    for caractere in str2.chars() {
        if let Some(contador) = contador_caracteres.get_mut(&caractere) {
            *contador -= 1;

            if *contador < 0 {
                return false;
            }
        } else {
            return false;
        }
    }

    return contador_caracteres.values().all(|&contador| contador == 0);
}

// Faça uma função que verifique se uma string possui todos os caracteres únicos

pub fn verificar_caracteres_unicos(texto: &str) -> bool {
    for caracter_buscado in texto.chars() {
        let mut contador = 0;
        for caracter_encontrado in texto.chars(){
            if caracter_buscado == caracter_encontrado {
                contador += 1;
            }
            if contador >= 2 {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let entrada1 = "abcdefghijk";
    let saida1 = verificar_caracteres_unicos(entrada1);
    println!("{} -> {}", entrada1, saida1);

    let entrada2 = "abcabcabc";
    let saida2 = verificar_caracteres_unicos(entrada2);
    println!("{} -> {}", entrada2, saida2);
}
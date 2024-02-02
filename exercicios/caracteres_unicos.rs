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

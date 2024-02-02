// Faça uma função para determinar se um número inteiro é um palindromo

pub fn palindromo(entrada: i32) -> bool {
    let entrada_string = entrada.to_string();
    let mut entrada_invertida = String::new();

    for caracter in entrada_string.chars().rev() {
        entrada_invertida.push(caracter);
    }

    return entrada_string == entrada_invertida;
}

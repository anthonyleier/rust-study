// Faça uma função para determinar se um número inteiro é um palindromo

pub fn palindromo(entrada: i32) -> bool {
    let entrada_string = entrada.to_string();
    let mut entrada_invertida = String::new();

    for caracter in entrada_string.chars().rev() {
        entrada_invertida.push(caracter);
    }

    return entrada_string == entrada_invertida;
}

fn main() {
    let entrada1 = 123456;
    let resultado1 = palindromo(entrada1);
    println!("{} -> {}", entrada1, resultado1);

    let entrada2 = 123321;
    let resultado2 = palindromo(entrada2);
    println!("{} -> {}", entrada2, resultado2);
}

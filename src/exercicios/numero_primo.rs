// Faça uma função que verifica se um número é primo, retornando "true" ou "false"

pub fn verificar_numero_primo(numero: i32) -> bool {
    if numero <= 1 {
        return false;
    }
    for i in 2..numero {
        if numero % i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let numero = 47;
    let eh_primo = verificar_numero_primo(numero);
    println!("Número: {} - Primo: {}", numero, eh_primo);

    let numero = 45;
    let eh_primo = verificar_numero_primo(numero);
    println!("Número: {} - Primo: {}", numero, eh_primo);
}

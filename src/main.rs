use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn somar_digitos() {
    let mut soma = 0;
    let mut valor_entrada = String::new();

    println!("Digite um número:");
    io::stdin()
        .read_line(&mut valor_entrada)
        .expect("Não foi possível ler a entrada");

    let mut valor_i32 = convert_to_int(&valor_entrada);

    while valor_i32 != 0 {
        let resto = valor_i32 % 10;

        soma += resto;

        valor_i32 = valor_i32 / 10;
    }

    println!("A soma é {}", soma);
}

fn main() {
    somar_digitos();
}

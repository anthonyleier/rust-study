use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn somar_digitos() {
    let mut soma = 0;
    let mut valor_entrada = String::new();

    println!("Digite o número:");
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

fn calcular_fatorial() {
    let mut fatorial = 1;
    let mut entrada_fatorial = String::new();

    println!("Digite o número:");
    io::stdin()
        .read_line(&mut entrada_fatorial)
        .expect("Não foi possível fazer a leitura");

    let mut entrada_fatorial_int = convert_to_int(&entrada_fatorial);
    while entrada_fatorial_int > 1 {
        fatorial = fatorial * entrada_fatorial_int;
        entrada_fatorial_int = entrada_fatorial_int - 1;
    }
    println!("O fatorial desse número é {}", fatorial);
}

fn verificar_alunos_recuperacao() {
    const NOTA_LIMITE: f64 = 7.0;
    let medias_alunos = [8.5, 3.4, 5.6, 8.9];

    let mut i = 0;
    while i < medias_alunos.len() {
        if medias_alunos[i] >= NOTA_LIMITE {
            println!("{} está aprovado", medias_alunos[i])
        } else {
            println!("{} está reprovado", medias_alunos[i])
        }
        i += 1;
    }
}

fn main() {
    verificar_alunos_recuperacao();
}

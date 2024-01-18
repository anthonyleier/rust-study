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
    let mut qtd_alunos = 0;
    let mut qtd_medias = String::new();
    let mut i = 0;

    println!("Quantos alunos são?");
    io::stdin()
        .read_line(&mut qtd_medias)
        .expect("Erro ao ler médias");

    while i < convert_to_int(&qtd_medias) {
        let mut media_aluno = String::new();
        println!("Qual a média desse aluno?");
        io::stdin()
            .read_line(&mut media_aluno)
            .expect("Erro ao ler médias");

        if convert_to_int(&media_aluno) >= 3 && convert_to_int(&media_aluno) < 6 {
            qtd_alunos += 1;
        }

        i += 1;
    }

    println!("Quantidade de alunos: {}", qtd_alunos);
}

fn encontrar_menor_divisor_comum(a: i32, b: i32) -> i32 {
    let mut i = 2;
    loop {
        if a % i == 0 && b % i == 0 {
            return i;
        }
        i += 1;
    }
}

fn encontrar_maior_numero(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn encontrar_maior_divisor_comum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn calcular_dobro(numero: i32) -> i32 {
    return numero * 2
}

fn main() {
    let dobro = calcular_dobro(2);
    println!("O dobro é {}", dobro);
}

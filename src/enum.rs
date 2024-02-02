enum Status {
    Inativo,
    Ativo,
    Pausado,
}

#[derive(Debug)]
enum Resultado {
    Ok(i32),
    Erro(String),
}

enum Pagamento {
    Dinheiro(f32),
    CartaoCredito(bool, f32),
    CartaoDebito(bool, f32),
}

fn imprimir_comprovante_pagamento(pagamento: Pagamento) {
    match pagamento {
        Pagamento::Dinheiro(valor) => {
            println!("Pagamento realizado em dinheiro no valor de R$ {}", valor)
        }
        Pagamento::CartaoCredito(aprovado, valor) => {
            println!(
                "Pagamento aprovado: {} - Realizado no cartão de crédito no valor de R$ {}",
                aprovado, valor
            )
        }
        Pagamento::CartaoDebito(aprovado, valor) => {
            println!(
                "Pagamento aprovado: {} - Realizado no cartão de débito no valor de R$ {}",
                aprovado, valor
            )
        }
    }
}

fn main() {
    let status_atual = Status::Inativo;

    match status_atual {
        Status::Ativo => println!("O processo está ativo"),
        Status::Inativo => println!("O processo está inativo"),
        Status::Pausado => println!("O processo está pausado"),
    }

    let mut resultado_atual = Resultado::Ok(200);
    println!("{:?}", resultado_atual);

    resultado_atual = Resultado::Erro("Processo não encontrado".to_string());
    println!("{:?}", resultado_atual);

    let pagamento1 = Pagamento::Dinheiro(50.89);
    let pagamento2 = Pagamento::CartaoCredito(true, 185.78);
    let pagamento3 = Pagamento::CartaoDebito(false, 98.23);

    imprimir_comprovante_pagamento(pagamento1);
    imprimir_comprovante_pagamento(pagamento2);
    imprimir_comprovante_pagamento(pagamento3);

    if let Status::Inativo = status_atual {
        println!("Sistema inativo")
    }
}

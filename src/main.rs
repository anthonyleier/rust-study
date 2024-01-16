use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    return x;
}

fn main() {
    println!("Insira o primeiro número:");
    let mut number1 = String::new();
    io::stdin()
        .read_line(&mut number1)
        .expect("Não foi possível ler o número 1");

    println!("Insira o segundo número:");
    let mut number2 = String::new();
    io::stdin()
        .read_line(&mut number2)
        .expect("Não foi possível ler o número 2");

    let result1 = convert_to_int(&number1);
    let result2 = convert_to_int(&number2);

    if result1 > result2 {
        println!("{} é maior que {}", result1, result2);
    } else if result2 > result1 {
        println!("{} é maior que {}", result2, result1);
    } else {
        println!("{} e {} são iguais", result1, result2)
    }
}

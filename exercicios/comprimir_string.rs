// Faça uma função que comprima strings, indicando o número de caracteres sequenciais

pub fn compress_string(input: &str) -> String {
    let mut compressed = String::new();
    let mut count = 0;

    for (i, current_char) in input.chars().enumerate() {
        if i == 0 || current_char == input.chars().nth(i - 1).unwrap() {
            count += 1;
        } else {
            compressed.push(current_char);
            compressed.push_str(&count.to_string());
            count = 1;
        }
    }

    // Adicionar a última sequência
    compressed.push(input.chars().last().unwrap());
    compressed.push_str(&count.to_string());

    // Retornar a string original se a versão comprimida não for menor
    if compressed.len() >= input.len() {
        return input.to_string();
    }

    compressed
}
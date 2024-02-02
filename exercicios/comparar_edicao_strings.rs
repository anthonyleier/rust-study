// Faça uma função que verifica se duas strings estão a uma edição de distância uma da outra

pub fn edicao_strings(str1: &str, str2: &str) -> bool {
    let len1 = str1.len();
    let len2 = str2.len();

    // Verifica se a diferença no comprimento é maior que 1
    if (len1 as i32 - len2 as i32).abs() > 1 {
        return false;
    }

    let mut edits = 0;
    let mut i = 0;
    let mut j = 0;

    // Percorre as strings para contar as edições
    while i < len1 && j < len2 {
        if str1.chars().nth(i) != str2.chars().nth(j) {
            edits += 1;

            // Se as strings têm comprimentos diferentes, avança apenas em uma delas
            if len1 > len2 {
                i += 1;
            } else if len1 < len2 {
                j += 1;
            } else {
                // Se as strings têm o mesmo comprimento, avança em ambas
                i += 1;
                j += 1;
            }
        } else {
            // Se os caracteres são iguais, avança em ambas as strings
            i += 1;
            j += 1;
        }

        // Se houver mais de uma edição, retorna falso
        if edits > 1 {
            return false;
        }
    }

    // Se as strings têm comprimentos diferentes e a diferença é de 1,
    // precisamos verificar se o último caractere é o mesmo nas duas strings
    if i < len1 || j < len2 {
        edits += 1;
    }

    // Retorna verdadeiro se houver no máximo uma edição
    edits <= 1
}

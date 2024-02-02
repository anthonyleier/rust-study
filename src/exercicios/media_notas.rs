// Faça uma função em Rust que recebe um vetor de notas como parâmetro e calcula a média dessas notas

pub fn calcular_media(vetor: &[i32]) -> f64 {
    let mut soma: i32 = 0;
    let mut media: f64 = 0.0;

    for item in vetor {
        soma += item;
    }
    media = soma as f64 / vetor.len() as f64;

    return media;
}

fn main(){
    let vetor = [5, 3, 10, 9, 4];
    let media = calcular_media(&vetor);
    println!("Vetor: {:?} - Média: {}", vetor, media)
}

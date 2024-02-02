// Esta tarefa requer que o candidato crie um programa em Rust que use funções, while e for loops para contar de 1 a 10

pub fn count(num: i32) {
    for i in 1..=num {
        println!("O número atual é {}", i);
    }
}

pub fn count_down(num: i32) {
    let mut i = num;
    while i > 0 {
        println!("O número atual é {}", i);
        i -= 1;
    }
}

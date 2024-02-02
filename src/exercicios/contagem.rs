// Faça funções, while e for loops para contar de 1 a 10 de forma crescente e decrescente

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

fn main() {
    count(10);
    count_down(20);
}

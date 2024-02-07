fn main() {
    let mut y = 5;
    {
        let y = 10;
        println!("O valor de y é: {}", y);
    }
    println!("O valor de y é {}", y);
    y = 6;
    println!("O valor de y é {}", y);
}

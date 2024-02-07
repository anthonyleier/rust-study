fn main() {
    let x = 5;

    {
        let x = x + 1;
        println!("Aqui X é {}", x);
    }

    println!("Aqui X é {}", x);
}

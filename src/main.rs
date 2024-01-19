mod tarefa8;

fn main() {
    let texto1: &str = "abc";
    let texto2: &str = "bca";
    let resultado = tarefa8::e_permutacao(&texto1, &texto2);
    println!("{}", resultado);
}

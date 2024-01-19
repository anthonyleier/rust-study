mod tarefa9;

fn main() {
    let texto1: &str = "josiani";
    let texto2: &str = "joseani";
    let resultado = tarefa9::edicao_strings(&texto1, &texto2);
    println!("{}", resultado);
}

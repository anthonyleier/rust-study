mod tarefa7;

fn main() {
    let texto: &str = "luiz";
    let resultado = tarefa7::verificar_caracteres_unicos(&texto);
    println!("{}", resultado);
}

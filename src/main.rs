mod tarefa10;

fn main() {
    let original_str = "aabcccccaaa";
    let compressed_str = tarefa10::compress_string(original_str);
    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);

    let other_str = "abcdefgh";
    let compressed_other = tarefa10::compress_string(other_str);
    println!("Original: {}", other_str);
    println!("Compressed: {}", compressed_other);
}

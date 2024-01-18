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

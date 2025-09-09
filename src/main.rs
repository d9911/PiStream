mod pi;

use std::io::{self, Write};

fn main() {
    println!("Вычисление числа π с использованием алгоритма Чудновских");
    print!("Введите количество знаков после запятой [по умолчанию: 1000]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let digits: usize = if input.is_empty() {
        1000
    } else {
        match input.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("Неверный ввод. Используется значение по умолчанию: 1000");
                1000
            }
        }
    };

    println!("Вычисление π с {} знаками после запятой...", digits);

    let pi_str = pi::compute_pi(digits);

    println!("π = {}", pi_str);
}

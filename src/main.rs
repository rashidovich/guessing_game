use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Привет прекрасная Аниса! ИГРА \"Одгадай число\"!");
    println!("Пожалуйста введите число от 1 до 98 (например 10, или 55)");

    let secret_number = rand::thread_rng().gen_range(1..99);
    let mut counter: u32 = 1;

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Не удалось прочитать.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Это не число. Попробуй еще раз");
                continue;
            }
        };

        println!("Попытка {}. Вы ввели: {}", counter, guess);
        increment(&mut counter);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое. Попробуй еще раз"),
            Ordering::Greater => println!("Слишком большое. Попробуй еще раз"),
            Ordering::Equal => {
                println!("Ура! Ты победил(а)!");
                break;
            }
        };
    }
}

fn increment(counter: &mut u32){
    *counter = *counter + 1;
}

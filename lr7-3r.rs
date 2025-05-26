use std::io;

// функция для получения первой цифры числа
fn first_digit(num: i32) -> i32 {
    // берем абсолютное значение на случай отрицательных чисел
    let mut num = num.abs();
    // последовательно делим число на 10, пока оно не станет меньше 10
    while num >= 10 {
        num /= 10;
    }
    num
}

// функция проверки, можно ли поставить число next после числа prev
fn can_place(prev: i32, next: i32) -> bool {
    // получаем последнюю цифру предыдущего числа
    let last_digit_prev = prev.abs() % 10;
    // получаем первую цифру следующего числа
    let first_digit_next = first_digit(next);
    // сравниваем цифры
    last_digit_prev == first_digit_next
}

// рекурсивная функция для поиска последовательности
fn find_sequence(
    numbers: &[i32],          // исходный массив чисел
    used: &mut Vec<bool>,     // отметки об использованных числах
    current: &mut Vec<i32>,   // текущая последовательность
    found: &mut bool,         // флаг, найдено ли решение
) {
    // если решение уже найдено, завершаем рекурсию
    if *found {
        return;
    }

    // если текущая последовательность содержит все числа, решение найдено
    if current.len() == numbers.len() {
        *found = true;
        println!("найдена подходящая последовательность: {:?}", current);
        return;
    }

    // перебираем все числа из исходного массива
    for i in 0..numbers.len() {
        // если число еще не использовано
        if !used[i] {
            // если последовательность пуста или числа можно поставить друг за другом
            if current.is_empty() || can_place(current[current.len() - 1], numbers[i]) {
                // помечаем число как использованное
                used[i] = true;
                // добавляем число в последовательность
                current.push(numbers[i]);

                // рекурсивно продолжаем поиск
                find_sequence(numbers, used, current, found);

                // если решение найдено, выходим
                if *found {
                    return;
                }

                // возвращаемся назад (backtracking)
                used[i] = false;
                current.pop();
            }
        }
    }
}

fn main() {
    println!("введите количество чисел:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("не удалось прочитать строку");

    // преобразуем ввод в число
    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ошибка ввода. будет использовано значение по умолчанию 5");
            5
        }
    };

    println!("введите числа через пробел({}):", n);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("не удалось прочитать строку");

    // разбиваем ввод на числа
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    // проверяем, что введено достаточно чисел
    if numbers.len() < n {
        println!("введено недостаточно чисел. будут использованы первые {}", n);
    }

    // берем только первые n чисел
    let numbers = &numbers[..n.min(numbers.len())];

    println!("введенные числа: {:?}", numbers);

    // создаем вектор для отметки использованных чисел
    let mut used = vec![false; numbers.len()];
    // вектор для текущей последовательности
    let mut current = Vec::new();
    // флаг, найдено ли решение
    let mut found = false;

    println!("ищем последовательность...");
    find_sequence(numbers, &mut used, &mut current, &mut found);

    // если решение не найдено
    if !found {
        println!("подходящая последовательность не найдена");
    }
}

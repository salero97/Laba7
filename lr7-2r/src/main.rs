use std::collections::HashMap;  
use std::sync::Mutex;  
use lazy_static::lazy_static;  

lazy_static! {
    // ключ - строка выражения, значение - вектор возможных результатов
    static ref CACHE: Mutex<HashMap<String, Vec<i32>>> = Mutex::new(HashMap::new());
}

// функция проверки, является ли строка числом
fn is_number(s: &str) -> bool {
    // пытаемся преобразовать строку в число типа i32
    // метод is_ok() возвращает true, если преобразование успешно
    s.parse::<i32>().is_ok()
}

// главная рекурсивная функция для вычисления всех возможных результатов
fn diff_ways_to_compute(expr: String) -> Vec<i32> {
    // блок для работы с кэшем
    {
        // получаем блокировку мьютекса для доступа к кэшу
        let cache = CACHE.lock().unwrap();
        // проверяем, есть ли уже результаты для этого выражения в кэше
        if let Some(res) = cache.get(&expr) {
            // если есть, возвращаем копию сохраненных результатов
            return res.clone();
        }
    }
    
    // создаем вектор для хранения результатов текущего выражения
    let mut results = Vec::new();
    
    // проверяем, является ли выражение простым числом
    if is_number(&expr) {
        // преобразуем строку в число и добавляем в результаты
        results.push(expr.parse::<i32>().unwrap());
        // сохраняем результат в кэше
        CACHE.lock().unwrap().insert(expr, results.clone());
        return results;
    }
    
    // перебираем все символы выражения вместе с их индексами
    for (i, c) in expr.chars().enumerate() {
        // проверяем, является ли текущий символ оператором
        if c == '+' || c == '-' || c == '*' {
            // разделяем выражение на левую часть (до оператора)
            let left_part = expr[..i].to_string();
            // разделяем выражение на правую часть (после оператора)
            let right_part = expr[i+1..].to_string();
            
            // рекурсивно вычисляем все возможные результаты для левой части
            let left_results = diff_ways_to_compute(left_part);
            // рекурсивно вычисляем все возможные результаты для правой части
            let right_results = diff_ways_to_compute(right_part);
            
            // комбинируем результаты левой и правой частей
            for &left in &left_results {  // для каждого результата левой части
                for &right in &right_results {  // для каждого результата правой части
                    // выполняем операцию в зависимости от текущего оператора
                    match c {
                        '+' => results.push(left + right),  // сложение
                        '-' => results.push(left - right),  // вычитание
                        '*' => results.push(left * right),  // умножение
                        _ => unreachable!(),  // этот случай никогда не должен произойти
                    }
                }
            }
        }
    }
    
    // сохраняем все полученные результаты в кэше
    CACHE.lock().unwrap().insert(expr, results.clone());
    results
}

fn main() {
    // примеры из задания
    let expr1 = "2-1-1".to_string();  // первое тестовое выражение
    let expr2 = "2*3-4*5".to_string();  // второе тестовое выражение
    
    // вычисляем все возможные результаты для первого выражения
    let res1 = diff_ways_to_compute(expr1);
    // вычисляем все возможные результаты для второго выражения
    let res2 = diff_ways_to_compute(expr2);
    
    // выводим результаты для первого выражения
    println!("ввод: \"2-1-1\"");
    println!("вывод: {:?}", res1);  // должно быть [0, 2]
    
    // выводим результаты для второго выражения
    println!("\nввод: \"2*3-4*5\"");
    println!("вывод: {:?}", res2);  // должно быть [-34, -14, -10, -10, 10]
    
    println!("\nвведите свое выражение (например 1+2*3):");
    let mut user_expr = String::new();  // создаем строку для хранения ввода
    // читаем ввод пользователя из стандартного потока ввода
    std::io::stdin().read_line(&mut user_expr).unwrap();
    user_expr = user_expr.trim().to_string();
    
    // вычисляем все возможные результаты для введенного выражения
    let user_res = diff_ways_to_compute(user_expr.clone());
    // выводим результаты
    println!("\nвсе возможные результаты для \"{}\": {:?}", user_expr, user_res);
}

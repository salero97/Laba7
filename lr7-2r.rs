use std::fmt; 
use std::io;  

// определяем структуру для хранения отрезков множества кантора
#[derive(Debug, Clone)] // debug - для отладки, clone - для клонирования
struct Segment {
    start: f64, // начало отрезка (дробное число от 0.0 до 1.0)
    end: f64,   // конец отрезка (дробное число от 0.0 до 1.0)
}

// реализуем отображение отрезка для удобного вывода
impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // форматируем вывод с точностью до 6 знаков после запятой
        write!(f, "[{:.6}, {:.6}]", self.start, self.end)
    }
}

fn draw_level(segments: &[Segment], level: usize) {
    const WIDTH: usize = 80;
    let mut line = vec![' '; WIDTH];
    
    for seg in segments {
        let start = (seg.start * WIDTH as f64) as usize; // как в C++
        let end = (seg.end * WIDTH as f64) as usize;
        
        for i in start..end {
            line[i] = '#'; // без проверки границ
        }
    }
   
    
    // вычисляем общую длину всех отрезков текущего уровня
    let total_length: f64 = segments.iter()
        .map(|s| s.end - s.start) // преобразуем каждый отрезок в его длину
        .sum(); // суммируем все длины
    
    // выводим информацию об уровне и графическое представление
    println!("n={} (отрезков: {}, длина: {:.6})", 
        level, segments.len(), total_length);
    println!("{}", line.iter().collect::<String>());
    println!(); // пустая строка для разделения
}

// функция для вывода координат всех отрезков уровня
fn print_coordinates(segments: &[Segment], level: usize) {
    println!("координаты отрезков для n={}:", level);
    
    // перебираем отрезки с индексами
    for (i, seg) in segments.iter().enumerate() {
        // выводим номер отрезка (начиная с 1) и его координаты
        println!("  {}: {}", i+1, seg);
    }
    println!(); // пустая строка для разделения
}

// функция для построения всех уровней множества кантора
fn build_cantor(levels: usize) -> Vec<Vec<Segment>> {
    // создаем вектор для хранения всех уровней
    let mut all_levels = Vec::new();
    
    // начальный уровень - один отрезок [0, 1]
    let mut current_level = vec![Segment { start: 0.0, end: 1.0 }];
    all_levels.push(current_level.clone()); // сохраняем копию
    
    // строим последующие уровни
    for level in 1..=levels {
        let mut next_level = Vec::new(); // создаем вектор для нового уровня
        
        // для каждого отрезка текущего уровня
        for seg in &current_level {
            let length = seg.end - seg.start; // вычисляем длину текущего отрезка
            
            // добавляем левый отрезок (первую треть)
            next_level.push(Segment {
                start: seg.start,
                end: seg.start + length / 3.0,
            });
            
            // добавляем правый отрезок (последнюю треть)
            next_level.push(Segment {
                start: seg.end - length / 3.0,
                end: seg.end,
            });
        }
        
        // обновляем текущий уровень на только что построенный
        current_level = next_level;
        // сохраняем копию нового уровня
        all_levels.push(current_level.clone());
    }
    
    // возвращаем все уровни
    all_levels
}

// функция для отрисовки всех уровней в одном общем графике
fn draw_all_levels(all_levels: &[Vec<Segment>]) {
    println!("общий график множества кантора:");
    println!("--------------------------------"); // ровно 84 символа
    
    for level in 0..=5 { // явно 6 уровней
        let segments = &all_levels[level];
        let mut line = vec![' '; 80];
        
        for seg in segments {
            let start = (seg.start * 80.0) as usize;
            let end = (seg.end * 80.0) as usize;
            
            for i in start..end {
                line[i] = '#';
            }
        }
        
        println!("n={} |{}|", level, line.iter().collect::<String>());
    }
    
    println!("--------------------------------");
}
// главная функция программы
fn main() {
    // выводим заголовок программы
    println!("построение множества кантора для уровней n=0 до n=5");
    println!("{}", "=".repeat(50));
    println!(); // пустая строка
    
    // строим 6 уровней множества кантора (0-5)
    let all_levels = build_cantor(5);
    
    // выводим каждый уровень с графиком и координатами
    for (level, segments) in all_levels.iter().enumerate() {
        draw_level(segments, level); // графическое представление
        print_coordinates(segments, level); // координаты отрезков
    }
    
    // выводим общий график всех уровней
    draw_all_levels(&all_levels);
    
    // завершающее сообщение
    println!("построение завершено. всего уровней: {}", all_levels.len());
}

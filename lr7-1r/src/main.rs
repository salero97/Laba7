use std::fmt;
use eframe::egui;
use eframe::epaint::{Color32, Stroke};

// Структура для хранения отрезка (сегмента) множества Кантора
#[derive(Debug, Clone, Copy)]
struct Segment {
    start: f64, // начальная координата сегмента
    end: f64,   // конечная координата сегмента
}

impl Segment {
    // Конструктор для создания нового сегмента
    fn new(start: f64, end: f64) -> Self {
        Segment { start, end }
    }

    // Метод для вычисления длины сегмента
    fn length(&self) -> f64 {
        self.end - self.start
    }
}

// Основная структура приложения, реализующая интерфейс eframe::App
struct CantorApp {
    levels: Vec<Vec<Segment>>, // вектор уровней, каждый уровень содержит вектор сегментов
    max_depth: usize,          // максимальная глубина рекурсии (уровень построения)
    built: bool,               // флаг, указывающий, построено ли множество
}

impl CantorApp {
    // Конструктор для создания нового приложения с заданной глубиной
    fn new(max_depth: usize) -> Self {
        CantorApp {
            levels: Vec::new(), // изначально уровни пусты
            max_depth,
            built: false,
        }
    }

    // Рекурсивная функция для построения множества Кантора
    fn build(&mut self, start: f64, end: f64, current_depth: usize) {
        // Если достигли максимальной глубины, прекращаем рекурсию
        if current_depth > self.max_depth {
            return;
        }

        // Если текущий уровень еще не создан, добавляем его
        if self.levels.len() <= current_depth {
            self.levels.push(Vec::new());
        }

        // Создаем сегмент текущего уровня
        let segment = Segment::new(start, end);
        // Добавляем сегмент в соответствующий уровень
        self.levels[current_depth].push(segment);

        // Вычисляем длину сегмента
        let length = segment.length();

        // Рекурсивно строим левый и правый сегменты по правилам множества Кантора
        self.build(start, start + length / 3.0, current_depth + 1);
        self.build(end - length / 3.0, end, current_depth + 1);
    }

    // Метод для вывода текстового представления множества в терминал
    fn print_text_representation(&self) {
        println!("построение множества кантора для уровней n=0 до n={}", self.max_depth);
        println!("==================================================\n");

        // Обработка каждого уровня
        for (level, segments) in self.levels.iter().enumerate() {
            const WIDTH: usize = 80; // ширина строки для визуализации
            let mut line = vec![' '; WIDTH]; // создаем строку из пробелов
            
            // Для каждого сегмента на уровне
            for segment in segments {
                // Вычисляем позиции начала и конца сегмента в строке
                let start = (segment.start * WIDTH as f64).round() as usize;
                let end = (segment.end * WIDTH as f64).round() as usize;
                // Заполняем соответствующие позиции символом '#'
                for i in start..end {
                    if i < WIDTH {
                        line[i] = '#';
                    }
                }
            }

            // Вычисляем суммарную длину сегментов на уровне
            let total_length: f64 = segments.iter().map(|s| s.length()).sum();

            // Выводим информацию о текущем уровне
            println!("n={} (отрезков: {}, длина: {:.6})", level, segments.len(), total_length);
            // Выводим визуализацию уровня
            println!("{}\n", line.iter().collect::<String>());

            // Выводим координаты каждого сегмента
            println!("координаты отрезков для n={}:", level);
            for (i, segment) in segments.iter().enumerate() {
                println!("  {}: [{:.6}, {:.6}]", i + 1, segment.start, segment.end);
            }
            println!();
        }

        // Итоговая информация
        println!("построение завершено. всего уровней: {}", self.max_depth + 1);
    }
}

// Реализация интерфейса eframe::App для интеграции с графическим интерфейсом
impl eframe::App for CantorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Панель управления слева
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Управление");
            // Ползунок для выбора глубины рекурсии
            ui.add(egui::Slider::new(&mut self.max_depth, 1..=10)
                .text("Глубина рекурсии"));
            // Кнопка для построения множества
            if ui.button("Построить").clicked() {
                self.levels.clear(); // очищаем предыдущие уровни
                self.build(0.0, 1.0, 0); // строим заново
                self.built = true; // отмечаем, что построено
                self.print_text_representation(); // выводим в терминал
            }
        });

        // Центральная панель для отображения графики
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Множество Кантора");
            // Получаем доступное пространство для рисования
            let available_size = ui.available_size();
            // Выделяем область для рисования
            let (response, painter) = ui.allocate_painter(
                available_size,
                egui::Sense::hover(),
            );
            // Заливаем фон черным
            painter.rect_filled(
                response.rect,
                0.0,
                Color32::from_gray(25),
            );

            // Если множество построено, рисуем его
            if self.built {
                // Высота каждого уровня
                let height = available_size.y / (self.max_depth as f32 + 1.0);
                // Для каждого уровня
                for (i, level) in self.levels.iter().enumerate() {
                    let y = height * (i as f32 + 0.5); // вертикальная позиция уровня
                    // Для каждого сегмента
                    for segment in level {
                        // Вычисляем координаты по горизонтали
                        let x1 = response.rect.left() + (segment.start * available_size.x as f64) as f32;
                        let x2 = response.rect.left() + (segment.end * available_size.x as f64) as f32;
                        // Рисуем линию сегмента
                        painter.line_segment(
                            [egui::pos2(x1, y), egui::pos2(x2, y)],
                            Stroke::new(2.0, Color32::WHITE),
                        );
                    }
                }
            }
        });
    }
}

// Точка входа в программу
fn main() -> eframe::Result<()> {
    // Настройки окна
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    // Запуск приложения
    eframe::run_native(
        "Множество Кантора",
        options,
        Box::new(|_cc| Box::new(CantorApp::new(5))), // создаем приложение с глубиной 5
    )
}
#include <SFML/Graphics.hpp>
#include <iostream>
#include <vector>
#include <iomanip>
#include <cmath>

using namespace std;
using namespace sf;

// структура для хранения отрезка (начало и конец)
struct Seg {
    double beg; // начало отрезка (0..1)
    double end; // конец отрезка (0..1)
};

// рекурсивная функция построения множества Кантора
void buildCant(vector<vector<Seg>>& lvls, double b, double e, int curlvl, int maxlvl) {
    // условие выхода из рекурсии
    if(curlvl > maxlvl) return;
    
    // добавляем новый уровень при необходимости
    if(lvls.size() <= curlvl) {
        lvls.push_back({});
    }
    
    // добавляем текущий отрезок в соответствующий уровень
    lvls[curlvl].push_back({b, e});
    
    // вычисляем длину текущего отрезка
    double len = e - b;
    
    // рекурсивно строим левую и правую части
    buildCant(lvls, b, b + len/3, curlvl + 1, maxlvl);
    buildCant(lvls, e - len/3, e, curlvl + 1, maxlvl);
}

// функция для вывода текстового представления уровня
void drawTxtLvl(const vector<Seg>& segs, int lvl) {
    const int w = 80; // ширина текстового представления
    string line(w, ' '); // создаем строку из пробелов

    // заполняем строку символами '#' в местах отрезков
    for(const auto& s : segs) {
        // переводим координаты в диапазон 0..w
        int beg = static_cast<int>(s.beg * w);
        int end = static_cast<int>(s.end * w);
        
        // заполняем отрезки символами
        for(int i = beg; i < end; ++i) {
            line[i] = '#';
        }
    }

    // выводим информацию об уровне
    cout << "n=" << lvl << " (отрезков: " << segs.size() 
         << ", длина: " << fixed << setprecision(6) 
         << (segs.empty() ? 0.0 : (segs[0].end - segs[0].beg) * segs.size()) << ")\n"
         << line << "\n";
}

// функция для вывода координат отрезков
void printCoords(const vector<Seg>& segs, int lvl) {
    cout << "координаты отрезков для n=" << lvl << ":\n";
    
    // выводим координаты каждого отрезка
    for(size_t i = 0; i < segs.size(); ++i) {
        cout << "  " << i+1 << ": [" << fixed << setprecision(6) 
             << segs[i].beg << ", " << segs[i].end << "]\n";
    }
    cout << "\n";
}

// функция для графического отображения множества Кантора
void drawCant(RenderWindow& win, const vector<vector<Seg>>& lvls) {
    // параметры отображения
    const float x = 50.f;    // начальная позиция по x
    const float y = 50.f;    // начальная позиция по y
    const float w = 700.f;   // максимальная ширина
    const float h = 10.f;    // высота линии
    const float step = 50.f; // расстояние между уровнями
    
    // очищаем окно
    win.clear(Color::Black);
    
    // рисуем все уровни
    for(size_t lvl = 0; lvl < lvls.size(); ++lvl) {
        // рисуем все отрезки текущего уровня
        for(const auto& s : lvls[lvl]) {
            // вычисляем позицию и длину отрезка
            float beg = x + static_cast<float>(s.beg) * w;
            float len = static_cast<float>(s.end - s.beg) * w;
            
            // создаем и настраиваем прямоугольник
            RectangleShape line(Vector2f(len, h));
            line.setPosition(beg, y + lvl * step);
            line.setFillColor(Color::White);
            
            // рисуем отрезок
            win.draw(line);
        }
    }
    
    // отображаем все нарисованное
    win.display();
}

int main() {
    const int maxlvl = 5; // максимальный уровень рекурсии
    vector<vector<Seg>> lvls; // вектор для хранения всех уровней
    
    // строим множество Кантора
    buildCant(lvls, 0.0, 1.0, 0, maxlvl);
    
    // выводим текстовую информацию
    cout << "построение множества кантора для уровней n=0 до n=" << maxlvl << "\n";
    cout << "==================================================\n\n";
    
    // выводим каждый уровень
    for(int lvl = 0; lvl <= maxlvl; ++lvl) {
        drawTxtLvl(lvls[lvl], lvl);
        printCoords(lvls[lvl], lvl);
    }
    cout << "построение завершено. всего уровней: " << maxlvl + 1 << "\n";
    
    // создаем графическое окно
    RenderWindow win(VideoMode(800, 400), "множество кантора");
    win.setFramerateLimit(60);
    
    // основной цикл программы
    while(win.isOpen()) {
        Event e;
        
        // обработка событий
        while(win.pollEvent(e)) {
            if(e.type == Event::Closed)
                win.close();
        }
        
        // рисуем множество Кантора
        drawCant(win, lvls);
    }
    
    return 0;
}

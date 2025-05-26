#include <iostream>
#include <vector>
#include <string>
#include <iomanip>
using namespace std;

// структура для хранения координат отрезков
struct Segment {
    double start; // начало отрезка (в диапазоне 0-1)
    double end;   // конец отрезка (в диапазоне 0-1)
};

// функция для отрисовки одного уровня множества кантора
void drawLevel(const vector<Segment>& segments, int level) {
    const int width = 80; // ширина графического представления в терминале
    
    // создаем строку из пробелов длиной width
    string line(width, ' ');
    
    // для каждого отрезка в текущем уровне
    for (const auto& seg : segments) {
        // переводим координаты отрезка в диапазон 0-ширина
        int start = static_cast<int>(seg.start * width);
        int end = static_cast<int>(seg.end * width);
        
        // рисуем отрезок, заполняя символами '#'
        for (int i = start; i < end; ++i) {
            line[i] = '#';
        }
    }
    
    // выводим информацию о текущем уровне: номер, количество отрезков, их суммарную длину
    cout << "n=" << level << " (отрезков: " << segments.size() 
         << ", длина: " << fixed << setprecision(6) 
         << (segments.empty() ? 0.0 : (segments[0].end - segments[0].start) * segments.size()) << ")" << endl;
    
    // выводим графическую строку
    cout << line << endl << endl;
}

// функция для вывода координат всех отрезков текущего уровня
void printCoordinates(const vector<Segment>& segments, int level) {
    cout << "координаты отрезков для n=" << level << ":" << endl;
    
    // для каждого отрезка выводим его номер и координаты
    for (size_t i = 0; i < segments.size(); ++i) {
        cout << "  " << i+1 << ": [" << fixed << setprecision(6) 
             << segments[i].start << ", " << segments[i].end << "]" << endl;
    }
    cout << endl;
}

// функция для отрисовки общего графика всех уровней
void drawAllLevels(const vector<vector<Segment>>& allLevels) {
    const int height = 6;    // количество уровней (0-5)
    const int width = 80;    // ширина графика
    
    cout << "общий график множества кантора:" << endl;
    cout << "--------------------------------" << endl;
    
    // для каждого уровня сверху вниз
    for (int level = 0; level < height; ++level) {
        string line(width, ' '); // создаем пустую строку из пробелов
        
        // заполняем отрезки текущего уровня
        for (const auto& seg : allLevels[level]) {
            int start = static_cast<int>(seg.start * width);
            int end = static_cast<int>(seg.end * width);
            
            // рисуем отрезок, заполняя символами '#'
            for (int i = start; i < end; ++i) {
                line[i] = '#';
            }
        }
        
        // выводим уровень и его графическое представление
        cout << "n=" << level << " |" << line << "|" << endl;
    }
    cout << "--------------------------------" << endl;
}

int main() {
    const int maxLevel = 5; // максимальный уровень (0-5)
    vector<vector<Segment>> allLevels; // вектор для хранения всех уровней
    
    // выводим сообщение о начале построения
    cout << "построение множества кантора для уровней n=0 до n=5" << endl;
    cout << "==================================================" << endl << endl;
    
    // создаем начальный уровень (n=0) - один отрезок [0,1]
    vector<Segment> currentLevel = {{0.0, 1.0}};
    // добавляем его в общий список уровней
    allLevels.push_back(currentLevel);
    
    // выводим первый уровень
    drawLevel(currentLevel, 0);
    printCoordinates(currentLevel, 0);
    
    // строим уровни 1-5
    for (int level = 1; level <= maxLevel; ++level) {
        vector<Segment> nextLevel; // вектор для следующего уровня
        
        // для каждого отрезка текущего уровня
        for (const auto& seg : currentLevel) {
            double length = seg.end - seg.start; // длина текущего отрезка
            
            // создаем два новых отрезка, исключая среднюю треть
            nextLevel.push_back({seg.start, seg.start + length/3});
            nextLevel.push_back({seg.end - length/3, seg.end});
        }
        
        // обновляем текущий уровень
        currentLevel = nextLevel;
        // добавляем его в список всех уровней
        allLevels.push_back(currentLevel);
        
        // выводим текущий уровень
        drawLevel(currentLevel, level);
        printCoordinates(currentLevel, level);
    }
    
    // выводим общий график всех уровней
    drawAllLevels(allLevels);
    
    // сообщение о завершении построения
    cout << "построение завершено. всего уровней: " << maxLevel + 1 << endl;
    return 0; // завершение программы
}

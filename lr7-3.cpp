#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

// функция для получения первой цифры числа
int getFirstDigit(int num) {
    // обрабатываем отрицательные числа
    num = abs(num);
    // пока число больше или равно 10, делим его на 10
    while (num >= 10) {
        num /= 10;
    }
    return num;
}

// функция для проверки возможности поставить число next после prev
bool canPlace(int prev, int next) {
    // получаем последнюю цифру предыдущего числа
    int lastDigitPrev = abs(prev) % 10;  // abs на случай отрицательных чисел
    // получаем первую цифру следующего числа
    int firstDigitNext = getFirstDigit(next);
    // сравниваем цифры
    return lastDigitPrev == firstDigitNext;
}

// рекурсивная функция построения последовательности
void findSequence(vector<int>& numbers, vector<bool>& used, vector<int>& current, bool& found) {
    // если последовательность уже найдена, завершаем рекурсию
    if (found) return;
    
    // если текущая последовательность содержит все числа, значит решение найдено
    if (current.size() == numbers.size()) {
        found = true;
        cout << "найдена подходящая последовательность: ";
        for (int num : current) {
            cout << num << " ";
        }
        cout << endl;
        return;
    }
    
    // перебираем все числа из исходного массива
    for (int i = 0; i < numbers.size(); ++i) {
        // если число еще не использовано
        if (!used[i]) {
            // если последовательность пустая ИЛИ можно поставить текущее число
            if (current.empty() || canPlace(current.back(), numbers[i])) {
                // добавляем число в последовательность
                used[i] = true;
                current.push_back(numbers[i]);
                
                // рекурсивно ищем дальше
                findSequence(numbers, used, current, found);
                
                // если решение найдено, выходим
                if (found) return;
                
                // возвращаемся назад (backtracking)
                used[i] = false;
                current.pop_back();
            }
        }
    }
}

int main() {
    int n;
    cout << "введите количество чисел: ";
    cin >> n;
    
    // проверка на корректный ввод
    while (n <= 0) {
        cout << "количество чисел должно быть положительным. повторите ввод: ";
        cin >> n;
    }
    
    vector<int> numbers(n);
    cout << "введите числа через пробел("<< n << "):";
    for (int i = 0; i < n; ++i) {
        cin >> numbers[i];
    }
    
    // массив для отметки использованных чисел
    vector<bool> used(n, false);
    // текущая последовательность
    vector<int> current;
    // флаг, найдено ли решение
    bool found = false;
    
    cout << "ищем последовательность..." << endl;
    findSequence(numbers, used, current, found);
    
    // если решение не найдено
    if (!found) {
        cout << "подходящая последовательность не найдена" << endl;
    }
    
    return 0;
}

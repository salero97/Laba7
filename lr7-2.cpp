#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

using namespace std;

// кэш для хранения уже вычисленных результатов подвыражений
unordered_map<string, vector<int>> cache;

// функция проверки, является ли строка числом
bool isNum(const string& s) {
    // перебираем все символы строки
    for (char c : s) {
        // если символ не цифра, возвращаем false
        if (!isdigit(c)) {
            return false;
        }
    }
    // если все символы цифры, возвращаем true
    return true;
}

// рекурсивная функция для вычисления всех возможных результатов
vector<int> diffWaysToCompute(string expr) {
    // проверяем, есть ли уже результат для этого выражения в кэше
    if (cache.find(expr) != cache.end()) {
        // если есть, возвращаем сохраненный результат
        return cache[expr];
    }
    
    // вектор для хранения результатов текущего выражения
    vector<int> res;
    
    // если выражение - просто число (без операторов)
    if (isNum(expr)) {
        // преобразуем строку в число и добавляем в результат
        res.push_back(stoi(expr));
        // сохраняем результат в кэше
        cache[expr] = res;
        // возвращаем результат
        return res;
    }
    
    // перебираем все символы выражения
    for (int i = 0; i < expr.size(); i++) {
        // текущий символ
        char c = expr[i];
        // если символ - оператор (+, -, *)
        if (c == '+' || c == '-' || c == '*') {
            // разделяем выражение на левую часть (до оператора)
            string leftStr = expr.substr(0, i);
            // разделяем выражение на правую часть (после оператора)
            string rightStr = expr.substr(i + 1);
            
            // рекурсивно вычисляем все возможные результаты для левой части
            vector<int> left = diffWaysToCompute(leftStr);
            // рекурсивно вычисляем все возможные результаты для правой части
            vector<int> right = diffWaysToCompute(rightStr);
            
            // комбинируем результаты левой и правой частей
            for (int l : left) {  // для каждого результата левой части
                for (int r : right) {  // для каждого результата правой части
                    // в зависимости от оператора выполняем соответствующую операцию
                    switch (c) {
                        case '+':
                            res.push_back(l + r);  // сложение
                            break;
                        case '-':
                            res.push_back(l - r);  // вычитание
                            break;
                        case '*':
                            res.push_back(l * r);  // умножение
                            break;
                    }
                }
            }
        }
    }
    
    // сохраняем все возможные результаты текущего выражения в кэше
    cache[expr] = res;
    // возвращаем результаты
    return res;
}

int main() {
    string expr;  // переменная для хранения введенного выражения
    
    // запрашиваем у пользователя ввод выражения
    cout << "введите выражение (например, 2-1-1 или 2*3-4*5): ";
    getline(cin, expr);  // считываем всю строку
    
    // вычисляем все возможные результаты
    vector<int> result = diffWaysToCompute(expr);
    
    // выводим результаты
    cout << "все возможные результаты:" << endl << "[";
    for (int i = 0; i < result.size(); i++) {
        cout << result[i];
        // если не последний элемент, добавляем запятую
        if (i != result.size() - 1) cout << ", ";
    }
    cout << "]" << endl;
    
    return 0;
}
/*
int main() {
    string expr1 = "2-1-1";
    vector<int> result1 = diffWaysToCompute(expr1);
    
    cout << "ввод: \"" << expr1 << "\"" << endl;
    cout << "вывод: [";
    for (int i = 0; i < result1.size(); i++) {
        cout << result1[i];
        if (i != result1.size() - 1) cout << ", ";
    }
    cout << "]" << endl;
    
    string expr2 = "2*3-4*5";
    vector<int> result2 = diffWaysToCompute(expr2);
    
    cout << "ввод: \"" << expr2 << "\"" << endl;
    cout << "вывод: [";
    for (int i = 0; i < result2.size(); i++) {
        cout << result2[i];
        if (i != result2.size() - 1) cout << ", ";
    }
    cout << "]" << endl;
    
    return 0;
}
 */

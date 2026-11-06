// notes 9: T = int makes const int**, which int** does not reach.
template <class T> int f(const T**) { return 1; }
int f(...) { return 2; }
int main() { int** p = nullptr; return f(p) == 2 ? 0 : 1; }

const int& pick(int a, int b) { int m = a > b ? a : b; return m; }
int main() { return pick(1, 2) == 2 ? 0 : 1; }

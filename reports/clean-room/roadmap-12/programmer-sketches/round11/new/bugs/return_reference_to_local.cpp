// A function that returns a reference to its own local.
const int& larger(int a, int b) { int m = a > b ? a : b; return m; }
int main() { const int& r = larger(3, 9); return r == 9 ? 0 : 1; }

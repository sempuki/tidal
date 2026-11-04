struct P { int a; int b; };
P f() { return {1, 2}; }
int main() { auto [x, y] = f(); return x - 1; }

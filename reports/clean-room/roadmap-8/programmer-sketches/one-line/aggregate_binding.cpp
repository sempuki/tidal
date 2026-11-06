struct P { int a; long b; };
int main() { P p{1, 2}; auto [x, y] = p; return x + y == 3 ? 0 : 1; }

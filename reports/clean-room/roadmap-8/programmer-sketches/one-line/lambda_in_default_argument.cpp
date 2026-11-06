int apply(int x, int (*f)(int) = [](int v) { return v + 1; }) { return f(x); }
int main() { return apply(1) == 2 ? 0 : 1; }

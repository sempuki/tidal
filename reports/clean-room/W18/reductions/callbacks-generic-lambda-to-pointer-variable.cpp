int main() { int (*f)(int) = [](auto x) { return x; }; return f(0); }

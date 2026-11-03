int main() { auto f = [](auto a, auto b) { return a < b; }; return f(1, 2) && !f(2.0, 1.0) ? 0 : 1; }

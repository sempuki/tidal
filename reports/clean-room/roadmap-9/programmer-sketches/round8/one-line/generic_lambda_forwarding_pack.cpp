int main() { auto n = [](auto&&... xs) { return sizeof...(xs); }; int a = 1; return n(a, 2, 'c') == 3 ? 0 : 1; }

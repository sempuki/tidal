int main() { auto l = [](auto... a) { return (a + ...); }; return l(1, -1); }

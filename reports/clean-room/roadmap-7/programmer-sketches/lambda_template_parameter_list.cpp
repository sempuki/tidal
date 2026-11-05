int main() {
    auto twice = []<class T>(T x) { return x + x; };
    return twice(21) == 42 ? 0 : 1;
}

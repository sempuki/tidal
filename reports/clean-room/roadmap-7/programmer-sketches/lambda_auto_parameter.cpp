int main() {
    auto twice = [](auto x) { return x + x; };
    return twice(21) == 42 ? 0 : 1;
}

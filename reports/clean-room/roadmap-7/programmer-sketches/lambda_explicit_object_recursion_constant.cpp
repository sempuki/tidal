constexpr int f10() {
    auto fib = [](this auto self, int n) -> int {
        return n < 2 ? n : self(n - 1) + self(n - 2);
    };
    return fib(10);
}
static_assert(f10() == 55);
int main() { return f10() == 55 ? 0 : 1; }

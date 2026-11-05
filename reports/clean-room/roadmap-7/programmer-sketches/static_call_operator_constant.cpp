struct Sum {
    static constexpr int operator()(int a, int b) { return a + b; }
};
static_assert(Sum{}(2, 3) == 5);
static_assert(Sum::operator()(2, 3) == 5);
int main() { return Sum{}(2, 3) == 5 ? 0 : 1; }

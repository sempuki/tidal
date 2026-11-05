struct Sum {
    template <class T>
    static constexpr T operator()(T a, T b) { return a + b; }
};
static_assert(Sum{}(2, 3) == 5);
static_assert(Sum::operator()(2, 3) == 5);
int main() { return Sum{}(2L, 3L) == 5 ? 0 : 1; }

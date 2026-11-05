struct Sum {
    template <class T>
    constexpr T operator()(T a) const { return a; }
};
static_assert(Sum{}(3) == 3);
int main() { return Sum{}(3) == 3 ? 0 : 1; }

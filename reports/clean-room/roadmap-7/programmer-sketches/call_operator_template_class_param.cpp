template <class T, int N> struct Box { T v[N]{}; };
struct Sum {
    template <class T, int N>
    constexpr T operator()(const Box<T, N>& b) const {
        T s{};
        for (auto v : b.v) s += v;
        return s;
    }
};
constexpr Box<int, 2> b{{1, 2}};
static_assert(Sum{}(b) == 3);
int main() { return Sum{}(b) == 3 ? 0 : 1; }

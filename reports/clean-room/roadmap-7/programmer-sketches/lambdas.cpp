// Callbacks the way a C++ programmer writes them day to day: a class
// that hands out lambdas capturing this and *this, a generic lambda with
// a template parameter list, a recursive lambda with an explicit object
// parameter, an immediately invoked lambda for a const value, a counter
// kept in a mutable capture, and attributes on the hot paths.
namespace std {
using size_t = decltype(sizeof 0);
template <class E>
class initializer_list {
    const E* b_;
    size_t n_;
    constexpr initializer_list(const E* b, size_t n) : b_(b), n_(n) {}
public:
    constexpr initializer_list() noexcept : b_(nullptr), n_(0) {}
    constexpr size_t size() const noexcept { return n_; }
    constexpr const E* begin() const noexcept { return b_; }
    constexpr const E* end() const noexcept { return b_ + n_; }
};
}

template <class F>
struct Callback {
    F f;
    template <class... A>
    constexpr auto operator()(A... a) const { return f(a...); }
};
template <class F> Callback(F) -> Callback<F>;

class Meter {
    int scale_;
    int total_ = 0;
public:
    constexpr explicit Meter(int s) : scale_(s) {}
    constexpr auto adder() { return [this](int v) { total_ += v * scale_; }; }
    constexpr auto snapshot() const { return [*this]() { return total_; }; }
    [[nodiscard]] constexpr int total() const { return total_; }
};

constexpr int sum_all(std::initializer_list<int> xs) {
    int s = 0;
    for (int x : xs) [[likely]] s += x;
    return s;
}

constexpr bool check() {
    // Immediately invoked lambda for a const value.
    const int base = [] {
        int b = 0;
        for (int i = 1; i <= 4; ++i) b += i;
        return b;
    }();
    if (base != 10) return false;

    auto twice = []<class T>(T x) { return x + x; };
    if (twice(21) != 42 || twice(1.5) != 3.0) return false;

    auto fib = [](this auto self, int n) -> int {
        return n < 2 ? n : self(n - 1) + self(n - 2);
    };
    if (fib(10) != 55) return false;

    auto counter = [n = 0]() mutable { return ++n; };
    counter();
    counter();
    if (counter() != 3) return false;

    Meter m(3);
    auto add = m.adder();
    auto before = m.snapshot();
    add(2);
    add(5);
    if (m.total() != 21 || before() != 0 || m.snapshot()() != 21) return false;

    Callback cb{[base](int a, int b) { return a * b + base; }};
    if (cb(2, 3) != 16) return false;

    return sum_all({1, 2, 3, 4}) == base;
}
static_assert(check());

int apply(int (*fp)(int), int v) noexcept { return fp(v); }

int main() {
    if (!check()) return 1;
    if (apply([](int v) { return v - 1; }, 5) != 4) return 2;
    static_assert(noexcept(apply(nullptr, 0)));
    auto ne = []() noexcept { return 1; };
    static_assert(noexcept(ne()));
    Meter m(2);
    auto add = m.adder();
    for (int i : {1, 2, 3}) add(i);
    return m.total() == 12 ? 0 : 3;
}

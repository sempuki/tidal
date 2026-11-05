// A fixed-size grid with the C++23 features a programmer reaches for:
// a subscript with two arguments, an explicit object parameter that
// forwards constness, a static call operator, if consteval, auto(x),
// the uz literal suffix, and a constexpr test of the whole thing.
namespace std {
using size_t = decltype(sizeof 0);
}

template <class T, std::size_t R, std::size_t C>
struct Grid {
    T cells[R * C]{};

    static constexpr std::size_t rows() { return R; }
    static constexpr std::size_t cols() { return C; }

    constexpr T& operator[](std::size_t r, std::size_t c) { return cells[r * C + c]; }
    constexpr const T& operator[](std::size_t r, std::size_t c) const { return cells[r * C + c]; }

    constexpr Grid transposed() const requires (R == C) {
        Grid out{};
        for (std::size_t r = 0; r < R; ++r)
            for (std::size_t c = 0; c < C; ++c)
                out[c, r] = (*this)[r, c];
        return out;
    }

    friend constexpr bool operator==(const Grid&, const Grid&) = default;
};

struct Sum {
    template <class T, std::size_t R, std::size_t C>
    static constexpr T operator()(const Grid<T, R, C>& g) {
        T s{};
        for (auto v : g.cells) s += v;
        return s;
    }
};

constexpr int scale_hint(int x) {
    if consteval {
        return x * 2;
    } else {
        return x * 3;
    }
}

template <class G>
constexpr G filled(int start) {
    G g{};
    for (auto r = 0uz; r < G::rows(); ++r)
        for (auto c = 0uz; c < G::cols(); ++c)
            g[r, c] = start++;
    return g;
}

// Takes a copy with auto(x) so the caller's grid stays as it was.
template <class G>
constexpr G doubled(const G& g) {
    auto copy = auto(g);
    for (auto& v : copy.cells) v = v * 2;
    return copy;
}

template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

using G3 = Grid<int, 3, 3>;

constexpr bool check() {
    G3 a = filled<G3>(1);
    if (a[0, 0] != 1 || a[2, 2] != 9) return false;
    G3 t = a.transposed();
    if (t[0, 1] != 4 || t[1, 0] != 2) return false;
    if (t.transposed() != a) return false;
    if (Sum{}(a) != 45) return false;
    if (Sum::operator()(doubled(a)) != 90) return false;
    if (a[1, 1] != 5) return false;
    const G3& ca = a;
    static_assert(same<decltype(ca[0, 0]), const int&>);
    static_assert(same<decltype(a[0, 0]), int&>);
    return true;
}
static_assert(check());
static_assert(scale_hint(1) == 2);

int main() {
    Grid<long, 2, 3> g = filled<Grid<long, 2, 3>>(10);
    long total = Sum{}(g);
    if (total != 10 + 11 + 12 + 13 + 14 + 15) return 1;
    if (scale_hint(1) != 3) return 2;
    G3 a = filled<G3>(0);
    if (doubled(a)[2, 2] != 16 || a[2, 2] != 8) return 4;
    return check() ? 0 : 5;
}

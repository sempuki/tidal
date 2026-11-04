// tidal-rules: LSUP-3 SB-5 NAM-21
// tidal-expect: run
// tidal-note: The tuple protocol example of the language-support page, unchanged, with a main that checks the bindings. size_t in the specializations is found in std after std::tuple_size<Range> (Q-0377).
namespace std {
using size_t = decltype(sizeof 0);
template <class E> struct tuple_size;
template <size_t I, class E> struct tuple_element;
}
class Range {
  int low, high;
public:
  constexpr Range(int l, int h) : low(l), high(h) {}
  template <std::size_t I> constexpr int get() const { return I == 0 ? low : high; }
};
template <> struct std::tuple_size<Range> { static constexpr size_t value = 2; };
template <> struct std::tuple_size<const Range> { static constexpr size_t value = 2; };
template <std::size_t I> struct std::tuple_element<I, Range> { using type = int; };
template <std::size_t I> struct std::tuple_element<I, const Range> { using type = int; };

auto [lo, hi] = Range{1, 5};        // decltype(lo) is int
const auto& [cl, ch] = Range{2, 7}; // decltype(cl) is int: the type as written

int main() { return lo == 1 && hi == 5 && cl == 2 && ch == 7 ? 0 : 1; }

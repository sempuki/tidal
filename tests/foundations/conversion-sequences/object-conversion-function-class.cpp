// tidal-rules: ICS-13
// tidal-expect: accept
// tidal-note: Both object parameters use Derived, so neither object sequence is better, and the second part int to int decides (ICS-37).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { constexpr operator int() const { return 1; } };
struct Derived : Base { constexpr operator long() const { return 2; } };
constexpr int chosen = Derived{};
static_assert(chosen == 1);

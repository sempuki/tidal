// tidal-rules: SB-5
// tidal-expect: accept
// tidal-note: The member lookup of get finds two templates, and one of them has a value as its first template parameter. So the member form e.get<i>() is used, and the free lib::get is not.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair {
  int first, second;
  template <class T> constexpr T get() const { return T(); }
  template <int I> constexpr int get() const { return I == 0 ? 7 : 8; }
};
template <decltype(sizeof 0) I> constexpr int get(const Pair& p) { return I == 0 ? p.first : p.second; }
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
constexpr int run() { auto [a, b] = lib::Pair{1, 2}; return a * 10 + b; }
static_assert(run() == 78);

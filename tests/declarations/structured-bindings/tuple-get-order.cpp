// tidal-rules: SB-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
int order[2];
int n = 0;
struct P {
  template <decltype(sizeof 0) I> int get() const { order[n++] = static_cast<int>(I); return 0; }
};
template <> struct std::tuple_size<P> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, P> { using type = int; };
int main() {
  auto [a, b] = P{};
  return n == 2 && order[0] == 0 && order[1] == 1 ? 0 : 1;
}

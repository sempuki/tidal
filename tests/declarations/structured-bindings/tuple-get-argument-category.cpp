// tidal-rules: SB-17
// tidal-expect: accept
// tidal-note: The argument of get is an lvalue when the hidden variable is an lvalue reference, and an xvalue otherwise, also when the hidden variable is not a reference.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair { int first, second; };
template <decltype(sizeof 0) I> constexpr int get(Pair&) { return 1; }
template <decltype(sizeof 0) I> constexpr int get(Pair&&) { return 2; }
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
constexpr int run() {
  lib::Pair p{0, 0};
  auto& [a, b] = p;
  auto&& [c, d] = static_cast<lib::Pair&&>(p);
  auto [e, f] = p;
  return a * 100 + c * 10 + e;
}
static_assert(run() == 122);

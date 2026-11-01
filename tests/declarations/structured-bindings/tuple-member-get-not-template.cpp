// tidal-rules: SB-5
// tidal-expect: accept
// tidal-note: The members named get are a plain function and, in Data, a data member. Neither is a template, so the call is get<i>(e) and finds lib::get.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair {
  int first, second;
  constexpr int get() const { return 7; }
};
struct Data { int first, second; int get = 0; };
template <decltype(sizeof 0) I> constexpr int get(const Pair& p) { return I == 0 ? p.first : p.second; }
template <decltype(sizeof 0) I> constexpr int get(const Data& d) { return I == 0 ? d.first : d.second; }
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
template <> struct std::tuple_size<lib::Data> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Data> { using type = int; };
constexpr int run() {
  auto [a, b] = lib::Pair{1, 2};
  auto [c, d] = lib::Data{3, 4, 0};
  return a * 1000 + b * 100 + c * 10 + d;
}
static_assert(run() == 1234);

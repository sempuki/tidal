// tidal-rules: SB-5 SB-14
// tidal-expect: reject
// tidal-error-clang: use of undeclared identifier .get
// tidal-error-gcc: .get. was not declared in this scope
// tidal-error-edg: suitable "get" function
// tidal-note: The only get is a member template whose first template parameter is a type. The member form does not apply, and argument-dependent lookup finds no get, so the binding is not valid.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib {
struct Pair {
  int first, second;
  template <class T> T get() const { return T(); }
};
}
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
void run() { auto [a, b] = lib::Pair{1, 2}; }

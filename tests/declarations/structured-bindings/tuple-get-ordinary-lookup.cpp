// tidal-rules: SB-14 SB-5
// tidal-expect: reject
// tidal-error-clang: use of undeclared identifier .get.
// tidal-error-gcc: .get. was not declared in this scope
// tidal-error-edg: get|no instance|undefined
// tidal-note: The free get is at global scope, outside the namespaces that lib::Pair adds. Only argument-dependent lookup takes part, so no get is found.
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
namespace lib { struct Pair { int first, second; }; }
template <decltype(sizeof 0) I> int get(const lib::Pair& p) { return I == 0 ? p.first : p.second; }
template <> struct std::tuple_size<lib::Pair> { static constexpr decltype(sizeof 0) value = 2; };
template <decltype(sizeof 0) I> struct std::tuple_element<I, lib::Pair> { using type = int; };
int use() { auto [a, b] = lib::Pair{1, 2}; return a + b; }

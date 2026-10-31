// tidal-rules: SB-14
// tidal-expect: reject
// tidal-error-clang: undefined template .std::tuple_element<0, P>
// tidal-error-gcc: incomplete type .struct std::tuple_element<0, P>
// tidal-error-edg: cannot instantiate .std::tuple_element
namespace std {
template <class T> struct tuple_size;
template <decltype(sizeof 0) I, class T> struct tuple_element;
}
struct P { template <decltype(sizeof 0) I> int get() const { return 0; } };
template <> struct std::tuple_size<P> { static constexpr decltype(sizeof 0) value = 1; };
P p;
auto [x] = p;

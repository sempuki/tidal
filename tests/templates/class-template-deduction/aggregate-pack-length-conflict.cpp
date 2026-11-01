// tidal-rules: CTAD-27 CTAD-9 CTAD-24
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-edg: no instance of constructor "Bases" matches
// tidal-note: The first base fixes T as X, Y, Z, so the last element must deduce Y. ToY only converts to Y, so its type gives a second value. GCC accepts (Q-0276).
template <class... T> struct List {};
struct X {};
struct Y {};
struct Z {};
struct ToY { constexpr operator Y() const { return {}; } };
template <class... T> struct Bases : List<T...>, T... {};
Bases b{List<X, Y, Z>{}, {}, ToY{}};

// tidal-rules: CTAD-27 CTAD-9 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Bases" matches
// tidal-note: The first base fixes T as X, Y, Z. The second element stands in the position of the base X, so its type Y gives a second value for that position.
template <class... T> struct List {};
struct X {};
struct Y {};
struct Z {};
template <class... T> struct Bases : List<T...>, T... {};
Bases b{List<X, Y, Z>{}, Y{}};

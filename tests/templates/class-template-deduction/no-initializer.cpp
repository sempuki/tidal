// tidal-rules: CTAD-6 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Box" matches
// tidal-note: Box declares a constructor, so it has no guide without parameters, and nothing gives T.
template <class T> struct Box { T v; Box(T x) : v(x) {} };
Box b;

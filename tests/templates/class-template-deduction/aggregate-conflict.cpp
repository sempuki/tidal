// tidal-rules: CTAD-9 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Twice" matches
// tidal-note: Both members have type T, and the elements give int and long.
template <class T> struct Twice { T x; T y; };
Twice t{1, 2L};

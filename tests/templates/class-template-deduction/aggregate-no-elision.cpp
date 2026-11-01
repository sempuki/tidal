// tidal-rules: CTAD-10 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Outer" matches
// tidal-note: The member In<T> has a dependent type and is no array, so its braces are not elided, and the list has one element too many.
template <class T> struct In { T x; T y; };
template <class T> struct Outer { In<T> in; };
Outer o{1, 2};

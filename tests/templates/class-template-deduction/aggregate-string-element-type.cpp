// tidal-rules: CTAD-9 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Bytes" matches
// tidal-note: For "abc" the parameter is a reference to const unsigned char[N]. The literal has the element type const char, so deduction fails.
template <int N> struct Bytes { unsigned char s[N]; };
Bytes u{"abc"};

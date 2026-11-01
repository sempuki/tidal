// tidal-rules: TARG-27
// tidal-expect: reject
// tidal-error-clang: found in multiple base classes of different types
// tidal-error-gcc: reference to .Base. is ambiguous
// tidal-error-edg: is ambiguous
template <class T> struct Base {};
struct Both : Base<int>, Base<char> { Base* p; };

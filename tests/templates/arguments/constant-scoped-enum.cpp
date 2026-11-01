// tidal-rules: TARG-13
// tidal-expect: reject
// tidal-error-clang: is not implicitly convertible to
// tidal-error-gcc: could not convert
// tidal-error-edg: is incompatible with template parameter of type
// tidal-note: A scoped enumeration does not convert to int implicitly.
enum class Level { low };
template <int N> struct Num {};
Num<Level::low> n;
